using Clam;
using Clam.FFI;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UIElements;

public class ClamGraphBuildMenu
{
    Button m_SelectClusters;
    Button m_CreateGraph;
    Button m_DestroyGraph;
    DropdownField m_ScoringSelector;
    //GameObject m_SpringPrefab;
    Slider m_EdgeScalar;
    Toggle m_ShowEdges;
    TextField m_MinDepth;

    GameObject m_GraphBuilder = null;
    Dictionary<string, GameObject> m_Graph;
    UIDocument m_Document;



    public ClamGraphBuildMenu(UIDocument document, string name)
    {
        m_Document = document;
        m_CreateGraph = document.rootVisualElement.Q<Button>("CreateClamGraphButton");
        m_DestroyGraph = document.rootVisualElement.Q<Button>("ResetClamGraph");
        m_SelectClusters = document.rootVisualElement.Q<Button>("SelectClamGraphClusters");
        m_EdgeScalar = document.rootVisualElement.Q<Slider>("ClamEdgeScalar");
        m_ScoringSelector = document.rootVisualElement.Q<DropdownField>("ScoringFunctionSelector");
        m_ShowEdges = document.rootVisualElement.Q<Toggle>("ShowEdgesToggle");
        m_MinDepth = document.rootVisualElement.Q<TextField>("GraphMinDepth");

        m_DestroyGraph.RegisterCallback<ClickEvent>(ResetCallback);

        //m_SpringPrefab = Resources.Load("Spring") as GameObject;

        m_CreateGraph.RegisterCallback<ClickEvent>(CreateGraphCallback);
        m_SelectClusters.RegisterCallback<ClickEvent>(SelectClustersForGraphCallback);

        m_ShowEdges.RegisterValueChangedCallback(ShowEdgesCallback);
        m_MinDepth.RegisterValueChangedCallback(MinDepthCallback);

        InitScoringSelector();

        if (m_GraphBuilder == null)
        {
            GameObject graphBuilderPrefab = Resources.Load("Graph") as GameObject;
            m_GraphBuilder = MenuEventManager.Instantiate(graphBuilderPrefab);
        }
    }

    void MinDepthCallback(ChangeEvent<string> changeEvent)
    {
        if (changeEvent.newValue.Length == 0)
        {
            return;
        }
        
        var textField = changeEvent.target as TextField;
        if (!UIHelpers.ValidateCharacters(changeEvent.newValue, "0123456789"))
        {
            textField.value = changeEvent.previousValue;
            return;
        }

        int minDepthValue = int.Parse(textField.value);

        if (minDepthValue < 0 || minDepthValue > NativeMethods.TreeHeight())
        {
            textField.value = changeEvent.previousValue;
            return;
        }
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
            var distanceMetric = Cakes.Tree.GetComponent<TreeCache>().m_TreeData.distanceMetric;
            string scoringFunctionString = scoringFunction.ToString();
            if (scoringFunctionString.Contains(distanceMetric.ToString()))
            {

                scoringFunctionStrings.Add(scoringFunctionString);
            }
        }

        m_ScoringSelector.choices = scoringFunctionStrings;
    }

    void SelectClustersForGraphCallback(ClickEvent evt)
    {
        if (MenuEventManager.instance.m_IsPhysicsRunning)
        {
            return;
        }
        foreach ((var id, var node) in Cakes.Tree.GetTree())
        {
            node.GetComponent<Node>().Deselect();
        }

        if (m_ScoringSelector.value == null)
        {
            UIHelpers.ShowErrorPopUP("Error: No scoring function selected");
            Debug.LogError("Error: No scoring function selected");
            return;
        }

        if (m_MinDepth.value.Length == 0)
        {
            UIHelpers.ShowErrorPopUP("Please enter a minimum depth for cluster selection");
            return;
        }

        var graphResult = Clam.FFI.NativeMethods.InitClamGraph((ScoringFunction)System.Enum.Parse(typeof(ScoringFunction), m_ScoringSelector.value),int.Parse(m_MinDepth.value), clusterSelector);
        if (graphResult != FFIError.Ok)
        {
            string errorMessage = "Error building graph (" + graphResult.ToString() + ")";
            Debug.LogError(errorMessage);
            UIHelpers.ShowErrorPopUP(errorMessage);
            return;
        }
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

        var numGraphComponentsLabel = m_Document.rootVisualElement.Q<Label>("NumGraphComponents");
        var numGraphComponents = NativeMethods.GetNumGraphComponents();
        numGraphComponentsLabel.text = "Num Components: " + numGraphComponents.ToString();

        var numGraphEdgesLabel = m_Document.rootVisualElement.Q<Label>("NumGraphEdges");
        numGraphEdgesLabel.text = "Num Edgesa: " + NativeMethods.GetNumGraphEdges().ToString();

        var numGraphClustersLabel = m_Document.rootVisualElement.Q<Label>("NumGraphClusters");
        numGraphClustersLabel.text = "Num Clusters: " + NativeMethods.GetGraphClusterCardinality().ToString();

    }

    void CreateGraphCallback(ClickEvent evt)
    {
        if (MenuEventManager.instance.m_IsPhysicsRunning)
        {
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

        MenuEventManager.SwitchState(Menu.DestroyGraph);
        MenuEventManager.SwitchState(Menu.DestroyTree);

        var selectedClusters = new Clam.FFI.ClusterData[m_Graph.Count];
        int i = 0;

        foreach (var (name, node) in m_Graph)
        {
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
        }
        //MenuEventManager.instance.m_IsPhysicsRunning = true;
        Debug.Log("finished setting up unity physics sim - passing to rust");
        

        m_GraphBuilder.GetComponent<GraphBuilder>().Init(selectedClusters, m_EdgeScalar.value, 500);
    }

    void ResetCallback(ClickEvent evt)
    {
        //if (!MenuEventManager.instance.m_IsPhysicsRunning)
        //{

        //    //MenuEventManager.SwitchState(Menu.DestroyGraph);

        m_GraphBuilder.GetComponent<GraphBuilder>().DestroyGraph();
        MenuEventManager.SwitchState(Menu.ResetTree);
        //    m_GraphBuilder.GetComponent<GraphBuilder>().DestroyGraph();
        //    Cakes.Tree.ResetTree();
        //}
        //else
        //{
        //    Debug.LogWarning("Cannot reset tree while physics is running");
        //}
    }

    void IncludeHiddenCallback(ClickEvent evt)
    {
        MenuEventManager.SwitchState(Menu.IncludeHidden);
    }

    public void clusterSelector(ref Clam.FFI.ClusterData nodeData)
    {
        if (Cakes.Tree.GetTree().TryGetValue(nodeData.id.AsString, out var node))
        {
            node.GetComponent<Node>().Select();
        }
        else
        {
            Debug.LogError("cluster not found");
        }
    }
}