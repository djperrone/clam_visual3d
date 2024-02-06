

using Clam;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UIElements;

public class GraphBuildMenu
{
    Button m_CreateGraph;
    Button m_DestroyGraph;
    Button m_HideSelected;
    Button m_HideOthers;
    Toggle m_IncludeHidden;
    GameObject m_SpringPrefab;
    Slider m_EdgeScalar;


    public GraphBuildMenu(UIDocument document, string name)
    {
        m_CreateGraph = document.rootVisualElement.Q<Button>("CreateGraphButton");
        m_DestroyGraph = document.rootVisualElement.Q<Button>("DestroyGraph");
        m_HideSelected = document.rootVisualElement.Q<Button>("HideSelected");
        m_HideOthers = document.rootVisualElement.Q<Button>("HideOthers");
        m_IncludeHidden = document.rootVisualElement.Q<Toggle>("ToggleHidden");
        m_EdgeScalar = document.rootVisualElement.Q<Slider>("EdgeScalar");

        m_HideOthers.RegisterCallback<ClickEvent>(HideOthersCallback);
        m_HideSelected.RegisterCallback<ClickEvent>(HideSelectedCallback);
        m_IncludeHidden.RegisterCallback<ClickEvent>(IncludeHiddenCallback);
        m_DestroyGraph.RegisterCallback<ClickEvent>(DestroyGraphCallback);

        m_SpringPrefab = Resources.Load("Spring") as GameObject;

        m_CreateGraph.RegisterCallback<ClickEvent>(CreateGraphCallback);
    }

    void HideOthersCallback(ClickEvent evt)
    {
        if (MenuEventManager.instance.m_IsPhysicsRunning)
        {
            return;
        }
        foreach (var (key, value) in Cakes.Tree.GetTree())
        {
            if (!value.GetComponent<Node>().Selected)
            {
                value.SetActive(false);
            }
        }
    }

    void HideSelectedCallback(ClickEvent evt)
    {
        foreach (var (key, value) in Cakes.Tree.GetTree())
        {
            if (value.GetComponent<Node>().Selected)
            {
                value.SetActive(false);
            }
        }
    }

    void ManuallyCreateGraphCallback(ClickEvent evt)
    {
        if (MenuEventManager.instance.m_IsPhysicsRunning)
        {
            return;
        }

        var graphResult = Clam.FFI.NativeMethods.InitClamGraphFromLeaves();

    }

    void ClusterSelectorCallback(ref Clam.FFI.ClusterData clusterData)
    {

    }

    void CreateGraphCallback(ClickEvent evt)
    {
        if (MenuEventManager.instance.m_IsPhysicsRunning)
        {
            return;
        }

        Cakes.BuildGraphWithinParams();

        MenuEventManager.SwitchState(Menu.DestroyGraph);
        MenuEventManager.SwitchState(Menu.DestroyTree);

        Clam.FFI.ClusterData[] nodes = new Clam.FFI.ClusterData[Cakes.Tree.GetTree().Count];
        int i = 0;

        foreach (var (name, node) in Cakes.Tree.GetTree())
        {
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
        }
        MenuEventManager.instance.m_IsPhysicsRunning = true;
        Debug.LogWarning("finished setting up unity physics sim - passing to rust");
        GameObject graphBuilderPrefab = Resources.Load("Graph") as GameObject;
        var graphBuilder = MenuEventManager.Instantiate(graphBuilderPrefab);
        graphBuilder.GetComponent<GraphBuilder>().Init(nodes, m_EdgeScalar.value, 500);
    }

    public void DestroyGraphCallback(ClickEvent evt)
    {
        MenuEventManager.SwitchState(Menu.DestroyTree);
        MenuEventManager.SwitchState(Menu.DestroyGraph);
    }

    void IncludeHiddenCallback(ClickEvent evt)
    {
        MenuEventManager.SwitchState(Menu.IncludeHidden);
    }
}