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
    Label m_ClusterInfo;
    Label m_ClusterInfoLabel;
    Dictionary<string, IntTextField> m_IntInputFields;
    Button m_DeselectAllClusters;

    // Start is called before the first frame update
    public ClusterMenu(UIDocument uiDoc)
    {
        m_UIDocument = uiDoc;

        m_ClusterInfo = m_UIDocument.rootVisualElement.Q<Label>("ClusterInfo");
        m_ClusterInfoLabel = m_UIDocument.rootVisualElement.Q<Label>("ClusterInfoLabel");
        InitClusterInfoLabel();

        var foundRoot = Clam.FFI.NativeMethods.GetRootData(out var dataWrapper);
        if (foundRoot == FFIError.Ok)
        {
            m_IntInputFields = new Dictionary<string, IntTextField>
            {
                { "Depth", new IntTextField("ClusterDepth", m_UIDocument, 0, Clam.FFI.NativeMethods.TreeHeight(), new Func<bool>(InputFieldChangeCallback)) },
                { "Cardinality", new IntTextField("ClusterCardinality", m_UIDocument, 0, dataWrapper.Data.cardinality, new Func<bool>(InputFieldChangeCallback)) },
            };
        }
        else
        {
            Debug.LogError("root not found");
        }

        m_DeselectAllClusters = m_UIDocument.rootVisualElement.Q<Button>("DeselectAllClusters");
        m_DeselectAllClusters.RegisterCallback<ClickEvent>(DeselectClustersCallback);
    }

    void DeselectClustersCallback(ClickEvent evt)
    {
        foreach ((var id, var cluster) in Cakes.Tree.GetTree())
        {
            cluster.GetComponent<Node>().Deselect();
        }
    }

    bool InputFieldChangeCallback()
    {
        foreach (var item in Cakes.Tree.GetTree().ToList())
        {
            var cluster = item.Value;
            var wrapper = new RustResourceWrapper<ClusterData>(ClusterData.Alloc(cluster.GetComponent<Node>().GetId()));
            if (wrapper.result == FFIError.Ok)
            {
                if (m_IntInputFields.TryGetValue("Depth", out var depthField))
                {
                    if (!depthField.IsWithinRange(wrapper.Data.depth))
                    {
                        cluster.GetComponent<Node>().Deselect();
                        continue;
                    }
                }

                if (m_IntInputFields.TryGetValue("Cardinality", out var cardField))
                {
                    if (!cardField.IsWithinRange(wrapper.Data.cardinality))
                    {
                        cluster.GetComponent<Node>().Deselect();
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
            stringBuilder.AppendLine("offset: ");
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
