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

    TreeLayout m_Layout;

    float m_MaxLFD;

    public TreeMenu(UIDocument uidocument)
    {
        m_UIDocument = uidocument;
        m_ColorOptions = m_UIDocument.rootVisualElement.Q<RadioButtonGroup>("TreeColorOptions");
        m_ColorOptions.choices = new List<string>() { "Label", "Cardinality", "Radius", "LFD" };//, "Component", "Ratios" };
        m_ColorOptions.RegisterValueChangedCallback(ColorChangeCallback);

        // root id
        var foundRoot = NativeMethods.GetRootData(out var rootData);
        m_Layout = new TreeLayout(rootData.Data.id.AsString);

        m_DepthField = new IntTextField("TreeDepth", m_UIDocument, 0, Clam.FFI.NativeMethods.TreeHeight(), InputFieldChangeCallback);

        m_ShowMore = m_UIDocument.rootVisualElement.Q<Button>("TreeDepthMoreButton");
        m_ShowLess = m_UIDocument.rootVisualElement.Q<Button>("TreeDepthLessButton");
        m_DepthValue = m_UIDocument.rootVisualElement.Q<Label>("TreeDepthValue");
        m_ShowMore.RegisterCallback<ClickEvent>(ShowMoreCallback);
        m_ShowLess.RegisterCallback<ClickEvent>(ShowLessCallback);

        m_MaxLFD = NativeMethods.MaxLFD();
    }

    void ShowMoreCallback(ClickEvent evt)
    {
        Debug.Log("Show more");
        m_Layout.ShowMore();
        m_DepthValue.text = m_Layout.CurrentDepth().ToString();
    }
    void ShowLessCallback(ClickEvent evt)
    {
        Debug.Log("Show less");
        m_Layout.ShowLess();
        m_DepthValue.text = m_Layout.CurrentDepth().ToString();
    }

    bool InputFieldChangeCallback()
    {
        Debug.Log("tree depth clbk");
        return true;
    }

    void ColorChangeCallback(ChangeEvent<int> changeEvent)
    {
        //var choices = new List<NodeVisitor>()
        //{
        //    ColorByLabel, ColorByCardinality, ColorByRadius,ColorByLFD
        //};
        //if (changeEvent != null && changeEvent.newValue >=0 && changeEvent.newValue < choices.Count)
        //{
        //    NativeMethods.ForEachDFT(choices[changeEvent.newValue]);
        //}
        //else
        //{
        //    Debug.LogError("invalid color choice");
        //}

        Debug.Log("color by " + m_ColorOptions.choices.ToList()[changeEvent.newValue]);
        if (changeEvent.newValue == 0)
        {
            NativeMethods.ColorClustersByLabel(ColorByLabel);
            Debug.Log("label");

        }
        else if (changeEvent.newValue == 1)
        {
            NativeMethods.ForEachDFT(ColorByCardinality);
            Debug.Log("cardinality");

        }
        else if (changeEvent.newValue == 2)
        {
            Debug.Log("radius");

            NativeMethods.ForEachDFT(ColorByRadius);
        }
        else if (changeEvent.newValue == 3)
        {
            Debug.Log("lfd");
            NativeMethods.ForEachDFT(ColorByLFD);
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
            Debug.LogError("cluster key not found - color filler - " + nodeData.id);
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
            Debug.LogError("cluster key not found - color filler - " + nodeData.id);
        }
    }
}
