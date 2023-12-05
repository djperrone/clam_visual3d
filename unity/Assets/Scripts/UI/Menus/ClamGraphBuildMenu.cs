using Clam;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UIElements;

public class ClamGraphBuildMenu
{
    Button m_SelectClusters;
    Button m_CreateGraph;
    Button m_DestroyGraph;
    DropdownField m_ScoringSelector;
    GameObject m_SpringPrefab;
    Slider m_EdgeScalar;
    Toggle m_ShowEdges;

    GameObject m_GraphBuilder = null;
    //Clam.FFI.ClusterData[] m_SelectedClusters;

    Dictionary<string, GameObject> m_Graph;

    //public GameObject m_GraphBuilderPrefab;

    public ClamGraphBuildMenu(UIDocument document, string name)
    {
        //m_GraphBuilderPrefab = graphBuilderPrefab;

        m_CreateGraph = document.rootVisualElement.Q<Button>("CreateClamGraphButton");
        m_DestroyGraph = document.rootVisualElement.Q<Button>("DestroyClamGraph");
        m_SelectClusters = document.rootVisualElement.Q<Button>("SelectClamGraphClusters");
        m_EdgeScalar = document.rootVisualElement.Q<Slider>("ClamEdgeScalar");
        m_ScoringSelector = document.rootVisualElement.Q<DropdownField>("ScoringFunctionSelector");
        m_ShowEdges = document.rootVisualElement.Q<Toggle>("ShowEdgesToggle");

        m_DestroyGraph.RegisterCallback<ClickEvent>(DestroyGraphCallback);

        m_SpringPrefab = Resources.Load("Spring") as GameObject;

        m_CreateGraph.RegisterCallback<ClickEvent>(CreateGraphCallback);
        m_SelectClusters.RegisterCallback<ClickEvent>(SelectClustersForGraphCallback);

        m_ShowEdges.RegisterValueChangedCallback(ShowEdgesCallback);

        InitScoringSelector();
    }

    void ShowEdgesCallback(ChangeEvent<bool> evt)
    {
        if (m_GraphBuilder != null)
        {
            m_GraphBuilder.GetComponent<GraphBuilder>().ToggleEdgeVisibility(evt.newValue);
        }
        else
        {
            Debug.Log("Warning: Graph builder does not exist yet");
        }
    }

    void InitScoringSelector()
    {
        List<string> scoringFunctionStrings = new List<string>();
        foreach (ScoringFunction scoringFunction in System.Enum.GetValues(typeof(ScoringFunction)))
        {
            string scoringFunctionString = scoringFunction.ToString();
            scoringFunctionStrings.Add(scoringFunctionString);
        }

        m_ScoringSelector.choices = scoringFunctionStrings;
    }

    

    void SelectClustersForGraphCallback(ClickEvent evt)
    {
        if (MenuEventManager.instance.m_IsPhysicsRunning)
        {
            //Debug.Log("Error physics already running");
            return;
        }
        foreach ((var id, var node) in Cakes.Tree.GetTree())
        {
            node.GetComponent<Node>().Deselect();
        }

        Clam.FFI.NativeMethods.InitClamGraph((ScoringFunction)System.Enum.Parse(typeof(ScoringFunction), m_ScoringSelector.value), clusterSelector);
        m_Graph = new Dictionary<string, GameObject>();

        foreach (var (id, node) in Cakes.Tree.GetTree())
        {
            if (node.GetComponent<Node>().IsSelected())
            {
                if (!node.activeSelf)
                {
                    node.SetActive(true);
                }
                m_Graph[id] = node;
            }
        }
    }


    void CreateGraphCallback(ClickEvent evt)
    {
        if (MenuEventManager.instance.m_IsPhysicsRunning)
        {
            //Debug.Log("Error physics already running");
            return;
        }

        foreach ((var id, var node) in Cakes.Tree.GetTree())
        {
            if (!m_Graph.ContainsKey(id))
            {
                GameObject.Destroy(node);
            }
        }

        Cakes.Tree.Set(m_Graph);

        //Cakes.BuildGraphWithinParams();



        //List<NodeDataUnity> nodes = new List<NodeDataUnity>();
        //int numSelected = 0;
        //foreach (var (name, node) in MenuEventManager.instance.GetTree())
        //{
        //    if (node.activeSelf && node.GetComponent<Node>().Selected)
        //    {
        //        numSelected++;
        //        //var x = Random.Range(0, 100);
        //        //var y = Random.Range(0, 100);
        //        //var z = Random.Range(0, 100);

        //        //node.GetComponent<Transform>().position = new Vector3(x, y, z);

        //        //nodes.Add(node.GetComponent<NodeScript>().ToUnityData());
        //    }
        //}

        MenuEventManager.SwitchState(Menu.DestroyGraph);
        MenuEventManager.SwitchState(Menu.DestroyTree);

        var selectedClusters = new Clam.FFI.ClusterData[m_Graph.Count];
        int i = 0;

        foreach (var (name, node) in m_Graph)
        {
            //if (node.activeSelf && node.GetComponent<Node>().Selected)
            {
                //numSelected++;
                var x = Random.Range(0, 100);
                var y = Random.Range(0, 100);
                var z = Random.Range(0, 100);

                node.GetComponent<Transform>().position = new Vector3(x, y, z);

                var result = Clam.FFI.NativeMethods.CreateClusterDataMustFree(node.GetComponent<Node>().GetId(), out var clusterData);
                if (result == FFIError.Ok)
                {
                    selectedClusters[i++] = clusterData;
                }
                else
                {
                    Debug.LogError("Node could not be found");
                    return;
                }
                //i++;

                //if (i == numSelected)
                //    break;
            }
        }
        //Clam.ClamFFI.InitForceDirectedSim(nodes, EdgeDrawer);
        MenuEventManager.instance.m_IsPhysicsRunning = true;
        Debug.LogWarning("finished setting up unity pgysics sim - passing to rust");
        //Clam.ClamFFI.LaunchPhysicsThread(nodes, m_EdgeScalar.value, 1000, EdgeDrawer, UpdatePhysicsSim);
        GameObject graphBuilderPrefab = Resources.Load("Graph") as GameObject;
        m_GraphBuilder = MenuEventManager.Instantiate(graphBuilderPrefab);
        m_GraphBuilder.GetComponent<GraphBuilder>().Init(selectedClusters, m_EdgeScalar.value, 500);

       

        //Clam.FFI.NativeMethods.RunForceDirectedSim(nodes, m_EdgeScalar.value, 500, EdgeDrawer);


        //for (int K = 0; K < selectedClusters.Length; K++)
        //{
        //    //ref var node = ref node1;
        //    //Debug.Log("freeing all nodes from physics sim");
        //    Clam.FFI.NativeMethods.DeleteClusterData(ref selectedClusters[K]);
        //}


    }
    //public void UpdatePhysicsSim(ref Clam.FFI.ClusterData nodeData)
    //{
    //    string id = nodeData.id.AsString;
    //    //Debug.Log("id of updated node is " + id);
    //    if (Cakes.Tree.GetTree().TryGetValue(id, out var node))
    //    {
    //        node.GetComponent<Node>().SetPosition(nodeData.pos.AsVector3);
    //    }
    //    else
    //    {
    //        Debug.Log("physics upodate key not found - " + id);
    //    }
    //}

    //public void EdgeDrawer(ref Clam.FFI.ClusterData nodeData)
    //{
    //    if (Cakes.Tree.GetTree().TryGetValue(nodeData.id.AsString, out var node))
    //    {
    //        if (Cakes.Tree.GetTree().TryGetValue(nodeData.message.AsString, out var other))
    //        {
    //            //Debug.Log("message from rust " + nodeData.message.AsString);
    //            //nodeData.SetMessage("hello world");
    //            //Clam.FFI.NativeMethods.SetMessage("hello world", out nodeData);
    //            //m_TempUI.AddEdge(node, other, 0);
    //            //Object springPrefab = Resources.Load("Spring");
    //            //var spring = SpringScript.CreateInstance(node, other, SpringScript.SpringType.Similarity);
    //            var spring = MenuEventManager.instance.MyInstantiate(m_SpringPrefab);

    //            spring.GetComponent<Edge>().InitLineRenderer(node, other, Edge.SpringType.Similarity);
    //        }
    //    }

    //}

    public void DestroyGraphCallback(ClickEvent evt)
    {
        //foreach(var (key, value) in MenuEventManager.instance.GetTree())
        //{
        //    value.SetActive(false);
        //}
        Debug.Log("is this running hello?00");
        MenuEventManager.SwitchState(Menu.DestroyTree);
        MenuEventManager.SwitchState(Menu.DestroyGraph);
    }



    void IncludeHiddenCallback(ClickEvent evt)
    {
        //foreach (var (key, value) in MenuEventManager.instance.GetTree())
        //{
        //    if (value.GetComponent<NodeScript>().Selected)
        //    {
        //        value.SetActive(false);
        //    }
        //}
        Debug.Log("toggled");
        MenuEventManager.SwitchState(Menu.IncludeHidden);
    }

    public void clusterSelector(ref Clam.FFI.ClusterData nodeData)
    {
        Debug.Log("clusterselecor call");
        if (Cakes.Tree.GetTree().TryGetValue(nodeData.id.AsString, out var node))
        {
            Debug.Log("seelcting for graph");
            node.GetComponent<Node>().Select();
        }
        else
        {
            //var cluster = Cakes.Tree.GetOrAdd(nodeData.id.AsString);
            Debug.LogError("cluster not found");
        }

    }

}