using Clam;
using Clam.FFI;
using System.Collections;
using System;
using System.Collections.Generic;
using System.Linq;
using Unity.VisualScripting;
using UnityEngine;
using UnityEngine.UIElements;
using System.IO;

public class ClamGraphBuildMenu
{
    Button m_SelectClusters;
    Button m_CreateGraph;
    Button m_DestroyGraph;
    DropdownField m_ScoringSelector;
    //GameObject m_SpringPrefab;
    TextField m_EdgeScalar;
    Toggle m_ShowEdges;
    TextField m_MinDepth;

    GameObject m_GraphBuilder = null;
    Dictionary<string, GameObject> m_Graph;
    UIDocument m_Document;
    int m_NumIters = 500;

    string m_OutTestFilePath;

    public ClamGraphBuildMenu(UIDocument document, string name)
    {
        m_Document = document;
        m_CreateGraph = document.rootVisualElement.Q<Button>("CreateClamGraphButton");
        m_DestroyGraph = document.rootVisualElement.Q<Button>("ResetClamGraph");
        m_SelectClusters = document.rootVisualElement.Q<Button>("SelectClamGraphClusters");
        m_EdgeScalar = document.rootVisualElement.Q<TextField>("ClamEdgeScalar");
        m_ScoringSelector = document.rootVisualElement.Q<DropdownField>("ScoringFunctionSelector");
        m_ShowEdges = document.rootVisualElement.Q<Toggle>("ShowEdgesToggle");
        m_MinDepth = document.rootVisualElement.Q<TextField>("GraphMinDepth");
        string m_TestOutputPath;

        m_DestroyGraph.RegisterCallback<ClickEvent>(ResetCallback);

        //m_SpringPrefab = Resources.Load("Spring") as GameObject;

        m_CreateGraph.RegisterCallback<ClickEvent>(CreateGraphCallback);
        m_SelectClusters.RegisterCallback<ClickEvent>(SelectClustersForGraphCallback);

        m_ShowEdges.RegisterValueChangedCallback(ShowEdgesCallback);
        m_MinDepth.RegisterValueChangedCallback(MinDepthCallback);
        m_EdgeScalar.RegisterValueChangedCallback(EdgeScalarCallback);

        InitScoringSelector();

        if (m_GraphBuilder == null)
        {
            GameObject graphBuilderPrefab = Resources.Load("Graph") as GameObject;
            m_GraphBuilder = MenuEventManager.Instantiate(graphBuilderPrefab);
        }

        InitLabelFilter();
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

    void EdgeScalarCallback(ChangeEvent<string> changeEvent)
    {
        if (changeEvent.newValue.Length == 0)
        {
            return;
        }

        var textField = changeEvent.target as TextField;

        if (changeEvent.newValue.Count(c => c == '.') > 1)
        {
            textField.value = changeEvent.previousValue;
            Debug.LogWarning("too many .s in edgesclar");
            return;
        }
        if (!UIHelpers.ValidateCharacters(changeEvent.newValue, ".0123456789"))
        {
            textField.value = changeEvent.previousValue;
            return;
        }

        //float minDepthValue = float.Parse(textField.value);

        //if (minDepthValue <= 0.0)
        //{
        //    textField.value = changeEvent.previousValue;
        //    return;
        //}
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

        m_Graph = new Dictionary<string, GameObject>();


        var graphResult = Clam.FFI.NativeMethods.InitClamGraph((ScoringFunction)System.Enum.Parse(typeof(ScoringFunction), m_ScoringSelector.value),int.Parse(m_MinDepth.value), graphFillerCallback);
        if (graphResult != FFIError.Ok)
        {
            string errorMessage = "Error building graph (" + graphResult.ToString() + ")";
            Debug.LogError(errorMessage);
            UIHelpers.ShowErrorPopUP(errorMessage);
            return;
        }

        foreach ((var id, var node) in Cakes.Tree.GetTree())
        {
            if (m_Graph.ContainsKey(id))
            {
                if (!node.activeSelf)
                {
                    node.SetActive(true);
                }
                node.GetComponent<Node>().Select();
            }
            else
            {
                node.GetComponent<Node>().Deselect();
                //node.SetActive(false);
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
        MenuEventManager.SwitchState(Menu.DestroyHierarchyEdges);

        m_OutTestFilePath = "triangle_test_results/" + Cakes.Tree.m_TreeData.dataName + "_" + Cakes.Tree.m_TreeData.cardinality + "_" + Cakes.Tree.m_TreeData.distanceMetric.ToString() + "_" + m_MinDepth.value.ToString() + ".csv";

        if (!File.Exists(m_OutTestFilePath))
        {
            // Create the file
            using (FileStream fs = File.Create(m_OutTestFilePath))
            {
                Console.WriteLine("File created successfully.");
            }
        }
        m_GraphBuilder.GetComponent<GraphBuilder>().Init(m_Graph, float.Parse(m_EdgeScalar.value), m_NumIters, m_OutTestFilePath);
    }

    void ResetCallback(ClickEvent evt)
    {
        var toggleMenu = m_Document.rootVisualElement.Q<VisualElement>("GraphLabelFilter");
        foreach (var child in toggleMenu.hierarchy.Children().ToList())
        {
            var c = child as Toggle;
            // why on earth is this necessary?... -
            // causes whole tree to appear with bugged clusters and edges
            c.SetValueWithoutNotify(true);
        }

        m_GraphBuilder.GetComponent<GraphBuilder>().DestroyGraph();
        MenuEventManager.SwitchState(Menu.ResetTree);
    }

    void IncludeHiddenCallback(ClickEvent evt)
    {
        MenuEventManager.SwitchState(Menu.IncludeHidden);
    }

    public void graphFillerCallback(ref Clam.FFI.ClusterData nodeData)
    {
        //if (Cakes.Tree.GetTree().TryGetValue(nodeData.id.AsString, out var node))
        //{
        //    //node.GetComponent<Node>().Select();
        //    m_Graph[nodeData.id.AsString] = node;
        //}
        //else
        //{

        //    Debug.LogError("cluster not found");
        //}
        var id = nodeData.id.AsString;
        var cluster = Cakes.Tree.GetOrAdd(id);

        m_Graph[id] = cluster;
    }

    void InitLabelFilter()
    {
        var baseMenu = m_Document.rootVisualElement.Q<VisualElement>("GraphLabelFilter");
        var numLabels = 2;
        if (Cakes.Tree.m_TreeData.dataName == "mnist")
        {
            numLabels = 10;
        }

        for (int i = 0; i < numLabels; i++)
        {
            Toggle t = new Toggle();
            t.style.width = Length.Percent(20);
            t.style.backgroundColor = UIHelpers.LabelColors()[i];
            t.text = i.ToString();
            t.value = true;
            t.name = i.ToString();
            t.RegisterValueChangedCallback(ToggleLabelCallback);


            //Toggle t = new Toggle(i.ToString());
            baseMenu.Add(t);
        }
    }
    
    void ToggleLabelCallback(ChangeEvent<bool> evt)
    {
        Debug.Log("triggered toggle callback");
        foreach ((var id, var cluster) in m_Graph)
        {
            var label = Clam.FFI.NativeMethods.GetClusterLabel(id);
            var target = evt.target as VisualElement;
            if (target.name == label.ToString())
            {
                cluster.SetActive(evt.newValue);
            }
        }
    }


    
}