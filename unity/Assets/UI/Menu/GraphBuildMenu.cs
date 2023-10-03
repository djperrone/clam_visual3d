

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
        Debug.Log("clicked hide others");

        if (MenuEventManager.instance.isPhysicsRunning)
        {
            {
                Debug.Log("Error cannot destroy graph while physics is running");
                return;
            }
        }
        foreach (var (key, value) in MenuEventManager.instance.GetTree())
        {
            if (!value.GetComponent<NodeScript>().Selected)
            {
                value.SetActive(false);
            }
        }
    }

    void HideSelectedCallback(ClickEvent evt)
    {
        Debug.Log("clicked hide others");
        foreach (var (key, value) in MenuEventManager.instance.GetTree())
        {
            if (value.GetComponent<NodeScript>().Selected)
            {
                value.SetActive(false);
            }
        }
    }

    void CreateGraphCallback(ClickEvent evt)
    {
        if (MenuEventManager.instance.isPhysicsRunning)
        {
            Debug.Log("Error physics already running");
            return;
        }

        MenuEventManager.SwitchState(Menu.DestroyGraph);

        List<NodeDataUnity> nodes = new List<NodeDataUnity>();
        foreach (var (name, node) in MenuEventManager.instance.GetTree())
        {
            if (node.activeSelf && node.GetComponent<NodeScript>().Selected)
            {
                var x = Random.Range(0, 100);
                var y = Random.Range(0, 100);
                var z = Random.Range(0, 100);

                node.GetComponent<Transform>().position = new Vector3(x, y, z);

                nodes.Add(node.GetComponent<NodeScript>().ToUnityData());
            }
        }
        //Clam.ClamFFI.InitForceDirectedSim(nodes, EdgeDrawer);
        MenuEventManager.instance.isPhysicsRunning = true;
        //Clam.ClamFFI.LaunchPhysicsThread(nodes, m_EdgeScalar.value, 1000, EdgeDrawer, UpdatePhysicsSim);
        Clam.ClamFFI.RunForceDirectedSim(nodes, m_EdgeScalar.value, 1000, EdgeDrawer);


    }
    public void UpdatePhysicsSim(ref NodeDataFFI nodeData)
    {
        string id = nodeData.id.AsString;
        Debug.Log("id of updated node is " + id);
        if (MenuEventManager.instance.GetTree().TryGetValue(id, out var node))
        {
            node.GetComponent<NodeScript>().SetPosition(nodeData.pos.AsVector3);
        }
        else
        {
            Debug.Log("physics upodate key not found - " + id);
        }
    }

    public void EdgeDrawer(ref NodeDataFFI nodeData)
    {
        if (MenuEventManager.instance.GetTree().TryGetValue(nodeData.id.AsString, out var node))
        {
            if (MenuEventManager.instance.GetTree().TryGetValue(nodeData.leftID.AsString, out var other))
            {
                //m_TempUI.AddEdge(node, other, 0);
                //Object springPrefab = Resources.Load("Spring");
                //var spring = SpringScript.CreateInstance(node, other, SpringScript.SpringType.Similarity);
                var spring = MenuEventManager.instance.MyInstantiate(m_SpringPrefab);

                spring.GetComponent<SpringScript>().InitLineRenderer(node, other, SpringScript.SpringType.Similarity);

            }
        }
    }

    public void DestroyGraphCallback(ClickEvent evt)
    {
        //foreach(var (key, value) in MenuEventManager.instance.GetTree())
        //{
        //    value.SetActive(false);
        //}
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
}