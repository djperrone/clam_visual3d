using Clam;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UIElements;

public class ClamGraphBuildMenu
{
    Button m_CreateGraph;
    Button m_DestroyGraph;
    GameObject m_SpringPrefab;
    Slider m_EdgeScalar;

    //public GameObject m_GraphBuilderPrefab;

    public ClamGraphBuildMenu(UIDocument document, string name)
    {
        //m_GraphBuilderPrefab = graphBuilderPrefab;

        m_CreateGraph = document.rootVisualElement.Q<Button>("CreateClamGraphButton");
        m_DestroyGraph = document.rootVisualElement.Q<Button>("DestroyClamGraph");
        m_EdgeScalar = document.rootVisualElement.Q<Slider>("ClamEdgeScalar");

        m_DestroyGraph.RegisterCallback<ClickEvent>(DestroyGraphCallback);

        m_SpringPrefab = Resources.Load("Spring") as GameObject;

        m_CreateGraph.RegisterCallback<ClickEvent>(CreateGraphCallback);
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
            node.GetComponent<Node>().Selected = false;
        }

        Clam.FFI.NativeMethods.InitClamGraph(clusterSelector);

        Cakes.BuildGraphWithinParams();

        //MenuEventManager.SwitchState(Menu.DestroyGraph);
        MenuEventManager.SwitchState(Menu.DestroyTree);

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
        Clam.FFI.ClusterData[] nodes = new Clam.FFI.ClusterData[Cakes.Tree.GetTree().Count];
        int i = 0;

        foreach (var (name, node) in Cakes.Tree.GetTree())
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
                    nodes[i++] = clusterData;
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
        var graphBuilder = MenuEventManager.Instantiate(graphBuilderPrefab);
        graphBuilder.GetComponent<GraphBuilder>().Init(nodes, m_EdgeScalar.value, 500);
        //Clam.FFI.NativeMethods.RunForceDirectedSim(nodes, m_EdgeScalar.value, 500, EdgeDrawer);


        //for (int K = 0; K < nodes.Length;K++)
        //{
        //    //ref var node = ref node1;
        //    //Debug.Log("freeing all nodes from physics sim");
        //    Clam.FFI.NativeMethods.DeleteClusterData(ref nodes[K]);
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
            Debug.LogError("cluster not found");
        }

    }

}