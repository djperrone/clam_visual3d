using Clam;
using Clam.FFI;
using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using UnityEngine;
using UnityEngine.UIElements;

public class ClusterMenu
{
    [SerializeField]
    VisualTreeAsset m_SafeInputFieldTemplate;

    private UIDocument m_UIDocument;
    //private Dictionary<string, GameObject> m_Tree;
    Label m_ClusterInfo;
    Label m_ClusterInfoLabel;
    Dictionary<string, IntTextField> m_IntInputFields;

    // Start is called before the first frame update
    public ClusterMenu(UIDocument uiDoc)
    {
        m_UIDocument = uiDoc;

        m_ClusterInfo = m_UIDocument.rootVisualElement.Q<Label>("ClusterInfo");
        m_ClusterInfoLabel = m_UIDocument.rootVisualElement.Q<Label>("ClusterInfoLabel");


        InitClusterInfoLabel();
        //var rightField = m_UIDocument.rootVisualElement.Q<VisualElement>("Right");

        var foundRoot = Clam.FFI.NativeMethods.GetRootData(out var dataWrapper);
        if (foundRoot == FFIError.Ok)
        {
            m_IntInputFields = new Dictionary<string, IntTextField>
            {
            //{ "Depth", new IntTextField("Depth", m_UIDocument, 0, 10, new Func<bool>(InputFieldChangeCallback)) },
                { "Depth", new IntTextField("ClusterDepth", m_UIDocument, 0, Clam.FFI.NativeMethods.TreeHeight(), new Func<bool>(InputFieldChangeCallback)) },
            //{ "Cardinality", new IntTextField("Cardinality", m_UIDocument, 0, 10, new Func<bool>(InputFieldChangeCallback)) },
                { "Cardinality", new IntTextField("ClusterCardinality", m_UIDocument, 0, dataWrapper.Data.cardinality, new Func<bool>(InputFieldChangeCallback)) },

            //{ "ArgRadius", new IntTextField("ArgRadius", rightField, 0, ClamFFI.ArgRadius(), new Func < bool >(InputFieldChangeCallback)) },
            //{ "ArgCenter", new IntTextField("ArgCenter", rightField, 0, ClamFFI.ArgCenter(), new Func < bool >(InputFieldChangeCallback)) }
            };
        }
        else
        {
            Debug.LogError("root not found");
        }
    }

    bool InputFieldChangeCallback()
    {
        foreach (var item in Cakes.Tree.GetTree().ToList())
        {
            var cluster = item.Value;
            //if (!cluster.activeSelf)
            //{
            //    continue;
            //}
            var wrapper = new RustResourceWrapper<ClusterData>(ClusterData.Alloc(cluster.GetComponent<Node>().GetId()));
            //var wrapper = Clam.FFI.NativeMethods.CreateClusterDataWrapper(cluster.GetComponent<Node>().GetId());

            //Clam.FFI.ClusterDataWrapper wrapper = new Clam.FFI.ClusterDataWrapper(cluster.GetComponent<Node>().ToNodeData());
            //Clam.FFI.NativeMethods.GetClusterData(wrapper);
            if (wrapper.result == FFIError.Ok)
            {
                if (m_IntInputFields.TryGetValue("Depth", out var depthField))
                {
                    //var range = textField.MinMaxRange();
                    if (!depthField.IsWithinRange(wrapper.Data.depth))
                    {
                        cluster.GetComponent<Node>().Deselect();
                        continue;
                    }
                }

                if (m_IntInputFields.TryGetValue("Cardinality", out var cardField))
                {
                    //var range = textField.MinMaxRange();
                    if (!cardField.IsWithinRange(wrapper.Data.cardinality))
                    {
                        cluster.GetComponent<Node>().Deselect();

                        //cluster.GetComponent<NodeScript>().Select();
                        continue;
                    }
                }
            }
            cluster.GetComponent<Node>().Select();
        }
        return true;
    }

    public void DisplayClusterInfo(Clam.FFI.ClusterData data)
    {
        if (m_ClusterInfo != null)
            m_ClusterInfo.text = data.GetInfoForUI();
    }

    public void ClearClusterInfo()
    {
        if (m_ClusterInfo != null)

            m_ClusterInfo.text = "";
    }

    public void InitClusterInfoLabel()
    {
        if (m_ClusterInfo != null)
        {
            StringBuilder stringBuilder = new StringBuilder();

            stringBuilder.AppendLine("id: ");
            stringBuilder.AppendLine("depth: ");
            stringBuilder.AppendLine("card: ");
            stringBuilder.AppendLine("radius: ");
            stringBuilder.AppendLine("lfd: ");
            stringBuilder.AppendLine("argC: ");
            stringBuilder.AppendLine("argR: ");

            m_ClusterInfoLabel.text = stringBuilder.ToString();
        }
    }

    public void SetSelectedClusterInfo(string value)
    {
        if (m_ClusterInfo != null)

            m_ClusterInfo.text = value;
    }

    public void Lock()
    {
        m_IntInputFields.ToList().ForEach(item => item.Value.Lock());
        var graphMenu = m_UIDocument.rootVisualElement.Q<VisualElement>("GraphMenuInstance");
        if (graphMenu != null)
        {
            graphMenu.Children().ToList().ForEach(c => c.focusable = false);
        }
    }

    public void UnLock()
    {

        m_IntInputFields.ToList().ForEach(item => item.Value.UnLock());
        //m_MenuSelector.Unlock();

        var graphMenu = m_UIDocument.rootVisualElement.Q<VisualElement>("GraphMenuInstance");
        if (graphMenu != null)
        {
            graphMenu.Children().ToList().ForEach(c => c.focusable = true);
        }
    }

    // Update is called once per frame
    void Update()
    {

    }
}
