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
        var foundRoot = NativeMethods.GetRootData(out var rootData);
        m_Layout = new TreeLayout(rootData.Data.id.AsString);

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
            var foundRoot = NativeMethods.GetRootData(out var rootData);
            m_Layout = new TreeLayout(rootData.Data.id.AsString);
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
            NativeMethods.ColorClustersByEntropy(ColorByLabel);
        }
        else
        {
            NativeMethods.ForEachDFT(choices[changeEvent.newValue]);
        }
    }

    void ColorByRadius(ref Clam.FFI.ClusterData nodeData)
    {
        bool hasValue = Cakes.Tree.GetTree().TryGetValue(nodeData.id.AsString, out var node);
        if (hasValue)
        {
            var rootFound = NativeMethods.GetRootData(out var rootWrapper);
            float ratio = 1.0f - (float)nodeData.radius / (float)rootWrapper.Data.radius;
            node.GetComponent<Node>().Deselect();
            node.GetComponent<Node>().SetColor(new Color(ratio, ratio, ratio));
        }
        else
        {
            Debug.LogWarning("cluster key not found - color by radius - " + nodeData.id);
        }
    }

    void ColorByCardinality(ref Clam.FFI.ClusterData nodeData)
    {
        bool hasValue = Cakes.Tree.GetTree().TryGetValue(nodeData.id.AsString, out var node);
        if (hasValue)
        {
            var rootFound = NativeMethods.GetRootData(out var rootWrapper);
            float ratio = 1.0f - (float)nodeData.cardinality / (float)rootWrapper.Data.cardinality;
            node.GetComponent<Node>().Deselect();
            node.GetComponent<Node>().SetColor(new Color(ratio, ratio, ratio));
        }
        else
        {
            Debug.LogWarning("cluster key not found - color by lfd - " + nodeData.id);
        }
    }

    void ColorByDepth(ref Clam.FFI.ClusterData nodeData)
    {
        bool hasValue = Cakes.Tree.GetTree().TryGetValue(nodeData.id.AsString, out var node);
        if (hasValue)
        {
            float ratio = 1.0f - (float)nodeData.depth / (float)NativeMethods.TreeHeight();
            node.GetComponent<Node>().Deselect();
            node.GetComponent<Node>().SetColor(new Color(ratio, ratio, ratio));
        }
        else
        {
            Debug.LogWarning("cluster key not found - color by lfd - " + nodeData.id);
        }
    }
    void ColorByLFD(ref Clam.FFI.ClusterData nodeData)
    {
        bool hasValue = Cakes.Tree.GetTree().TryGetValue(nodeData.id.AsString, out var node);
        if (hasValue)
        {
            float ratio = 1.0f - nodeData.lfd / m_MaxLFD;
            node.GetComponent<Node>().SetColor(new Color(ratio, ratio, ratio));
        }
        else
        {
            Debug.LogWarning("cluster key not found - color by lfd - " + nodeData.id);
        }
    }

    void ColorByVertexDegree(ref Clam.FFI.ClusterData nodeData)
    {
        bool hasValue = Cakes.Tree.GetTree().TryGetValue(nodeData.id.AsString, out var node);
        if (hasValue)
        {
            int maxVertexDegree = NativeMethods.MaxVertexDegree();
            int vertexDegree = NativeMethods.VertexDegree(nodeData.id.AsString);
            float ratio = 1.0f - (float)vertexDegree / (float)maxVertexDegree;
            node.GetComponent<Node>().SetColor(new Color(ratio, ratio, ratio));
        }
        else
        {
            Debug.LogWarning("cluster key not found - color by lfd - " + nodeData.id);
        }
    }

    void ColorByLabel(ref Clam.FFI.ClusterData nodeData)
    {
        bool hasValue = Cakes.Tree.GetTree().TryGetValue(nodeData.id.AsString, out var node);
        if (hasValue)
        {
            //Debug.Log("setting color to" + nodeData.color.AsVector3.ToString());
            node.GetComponent<Node>().Deselect();
            node.GetComponent<Node>().SetColor(nodeData.color.AsColor);
        }
        else
        {
            Debug.LogWarning("cluster key not found - color by label - " + nodeData.id);
        }
    }
}
