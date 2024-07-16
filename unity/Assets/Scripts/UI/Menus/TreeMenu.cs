using Clam.FFI;
using Clam;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UIElements;
using System;
using System.Linq;

public class TreeMenu
{
    RadioButtonGroup m_ColorOptions;
    private UIDocument m_UIDocument;

    IntTextField m_DepthField;
    Label m_DepthValue;
    Button m_ShowLess;
    Button m_ShowMore;
    Button m_ResetLayout;

    TreeLayout m_Layout;

    float m_MaxLFD;
    int m_MaxVertexDegree;

    public TreeMenu(UIDocument uidocument)
    {
        m_UIDocument = uidocument;
        m_ColorOptions = m_UIDocument.rootVisualElement.Q<RadioButtonGroup>("TreeColorOptions");
        m_ColorOptions.choices = new List<string>() { "Label", "Cardinality", "Radius", "LFD", "Depth", "VertexDegree" };//, "Component", "Ratios" };
        m_ColorOptions.RegisterValueChangedCallback(ColorChangeCallback);

        // root id
        (FFIError err, ClusterData rootData) = NativeMethods.GetRootData();
        if (err == FFIError.Ok)
        {
            m_Layout = new TreeLayout(new ClusterID(rootData.offset, rootData.cardinality));

        }
        else
        {
            Debug.LogError("Erro finding root" + err.ToString());
        }

        m_DepthField = new IntTextField("TreeDepth", m_UIDocument, 0, Clam.FFI.NativeMethods.TreeHeight(), InputFieldChangeCallback);

        m_ShowMore = m_UIDocument.rootVisualElement.Q<Button>("TreeDepthMoreButton");
        m_ResetLayout = m_UIDocument.rootVisualElement.Q<Button>("ResetTreeLayout");
        m_ShowLess = m_UIDocument.rootVisualElement.Q<Button>("TreeDepthLessButton");
        m_DepthValue = m_UIDocument.rootVisualElement.Q<Label>("TreeDepthValue");
        m_ShowMore.RegisterCallback<ClickEvent>(ShowMoreCallback);
        m_ShowLess.RegisterCallback<ClickEvent>(ShowLessCallback);
        m_ResetLayout.RegisterCallback<ClickEvent>(ResetCallback);

        m_MaxLFD = NativeMethods.MaxLFD();
        m_MaxVertexDegree = NativeMethods.MaxVertexDegree();
        var depthLabel = m_UIDocument.rootVisualElement.Q<Label>("TreeDepthButtonLabel");
        depthLabel.text = "visible depth (max " + Clam.FFI.NativeMethods.TreeHeight().ToString() + "):";
        m_DepthValue.text = m_Layout.CurrentDepth().ToString();

        MenuEventManager.StartListening(Menu.ResetTree, ResetTree);

    }

    void ResetCallback(ClickEvent evt)
    {
        ResetTree();
    }

    void ResetTree()
    {
        if (!MenuEventManager.instance.m_IsPhysicsRunning)
        {

            //MenuEventManager.SwitchState(Menu.DestroyGraph);
            //MenuEventManager.SwitchState(Menu.DestroyTree);
            Cakes.Tree.ResetTree();
            (FFIError err, ClusterData rootData) = NativeMethods.GetRootData();
            m_Layout = new TreeLayout(rootData.ID());
            var depthLabel = m_UIDocument.rootVisualElement.Q<Label>("TreeDepthButtonLabel");
            depthLabel.text = "visible depth (max " + Clam.FFI.NativeMethods.TreeHeight().ToString() + "):";
            m_DepthValue.text = m_Layout.CurrentDepth().ToString();

        }
        else
        {
            Debug.LogWarning("Cannot reset tree while physics is running");
        }
    }

    void ShowMoreCallback(ClickEvent evt)
    {
        m_Layout.ShowMore();
        m_DepthValue.text = m_Layout.CurrentDepth().ToString();
    }
    void ShowLessCallback(ClickEvent evt)
    {
        m_Layout.ShowLess();
        m_DepthValue.text = m_Layout.CurrentDepth().ToString();
    }

    bool InputFieldChangeCallback()
    {
        return true;
    }

    void ColorChangeCallback(ChangeEvent<int> changeEvent)
    {
        var choices = new List<NodeVisitor>()
        {
            ColorByLabel, ColorByCardinality, ColorByRadius,ColorByLFD, ColorByDepth, ColorByVertexDegree
        };

        if (changeEvent.newValue == 0)
        {
            NativeMethods.ColorClustersByDominantLabel(ColorByLabel);
        }
        else
        {
            NativeMethods.ForEachDFT(choices[changeEvent.newValue], new ClusterID(0, NativeMethods.TreeCardinality()));
        }
    }

    void ColorByRadius(ref Clam.FFI.ClusterData nodeData)
    {
        bool hasValue = Cakes.Tree.GetTree().TryGetValue(nodeData.ID_AsTuple(), out var node);
        if (hasValue)
        {
            (FFIError err, ClusterData rootData) = NativeMethods.GetRootData();


            float ratio = 1.0f - nodeData.radius / rootData.radius;
            node.GetComponent<Node>().Deselect();
            node.GetComponent<Node>().SetColor(new Color(ratio, ratio, ratio));
        }
        else
        {
            Debug.LogWarning("cluster key not found - color by radius - " + nodeData.ID_AsTuple());
        }
    }

    void ColorByCardinality(ref Clam.FFI.ClusterData nodeData)
    {
        bool hasValue = Cakes.Tree.GetTree().TryGetValue(nodeData.ID_AsTuple(), out var node);
        if (hasValue)
        {
            //var rootFound = NativeMethods.GetRootData(out var rootWrapper);
            (FFIError err, ClusterData rootData) = NativeMethods.GetRootData();

            float ratio = 1.0f - (float)nodeData.cardinality / (float)rootData.cardinality;
            node.GetComponent<Node>().Deselect();
            node.GetComponent<Node>().SetColor(new Color(ratio, ratio, ratio));
        }
        else
        {
            Debug.LogWarning("cluster key not found - color by card - " + nodeData.ID_AsTuple());
        }
    }

    void ColorByDepth(ref Clam.FFI.ClusterData nodeData)
    {
        bool hasValue = Cakes.Tree.GetTree().TryGetValue(nodeData.ID_AsTuple(), out var node);
        if (hasValue)
        {
            float ratio = 1.0f - (float)nodeData.depth / (float)NativeMethods.TreeHeight();
            node.GetComponent<Node>().Deselect();
            node.GetComponent<Node>().SetColor(new Color(ratio, ratio, ratio));
        }
        else
        {
            Debug.LogWarning("cluster key not found - color by lfd - " + nodeData.ID_AsTuple());
        }
    }
    void ColorByLFD(ref Clam.FFI.ClusterData nodeData)
    {
        bool hasValue = Cakes.Tree.GetTree().TryGetValue(nodeData.ID_AsTuple(), out var node);
        if (hasValue)
        {
            float ratio = 1.0f - nodeData.lfd / m_MaxLFD;
            node.GetComponent<Node>().SetColor(new Color(ratio, ratio, ratio));
        }
        else
        {
            Debug.LogWarning("cluster key not found - color by lfd - " + nodeData.ID_AsTuple());
        }
    }

    void ColorByVertexDegree(ref Clam.FFI.ClusterData nodeData)
    {
        bool hasValue = Cakes.Tree.GetTree().TryGetValue(nodeData.ID_AsTuple(), out var node);
        if (hasValue)
        {
            int maxVertexDegree = NativeMethods.MaxVertexDegree();
            int vertexDegree = NativeMethods.VertexDegree(nodeData.ID_AsTuple());
            float ratio = 1.0f - (float)vertexDegree / (float)maxVertexDegree;
            node.GetComponent<Node>().SetColor(new Color(ratio, ratio, ratio));
        }
        else
        {
            Debug.LogWarning("cluster key not found - color by lfd - " + nodeData.ID_AsTuple());
        }
    }

    void ColorByLabel(ref Clam.FFI.ClusterData nodeData)
    {
        bool hasValue = Cakes.Tree.GetTree().TryGetValue(nodeData.ID_AsTuple(), out var node);
        if (hasValue)
        {
            //Debug.Log("setting color to" + nodeData.color.AsVector3.ToString());
            node.GetComponent<Node>().Deselect();
            node.GetComponent<Node>().SetColor(nodeData.color.AsColor);
        }
        else
        {
            Debug.LogWarning("cluster key not found - color by label - " + nodeData.ID_AsTuple());
        }
    }
}
