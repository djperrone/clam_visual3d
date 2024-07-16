//using Clam;
//using Clam.FFI;
//using System;
//using System.Collections.Generic;
//using System.Linq;
//using System.Text;
//using Unity.VisualScripting;
//using UnityEditor;
//using UnityEngine;
//using UnityEngine.UI;
//using UnityEngine.UIElements;
//using static UnityEngine.Rendering.DebugUI;

//public class ClusterUI_View : MonoBehaviour
//{
//    [SerializeField]
//    VisualTreeAsset m_SafeInputFieldTemplate;

//    private UIDocument m_UIDocument;
//    Label m_ClusterInfo;
//    Label m_ClusterInfoLabel;
//    RadioButtonGroup m_ColorOptions;
//    MenuSelector m_MenuSelector;
//    Dictionary<string, IntTextField> m_IntInputFields;

//    public VisualTreeAsset m_GraphBuilder;
//    public VisualTreeAsset m_TreeSettings;


//    public void Start()
//    {
//        m_UIDocument = GetComponent<UIDocument>();

//        m_ClusterInfo = m_UIDocument.rootVisualElement.Q<Label>("ClusterInfo");
//        m_ClusterInfoLabel = m_UIDocument.rootVisualElement.Q<Label>("ClusterInfoLabel");
//        m_ColorOptions = m_UIDocument.rootVisualElement.Q<RadioButtonGroup>("ColorOptions");
//        m_ColorOptions.RegisterValueChangedCallback(ColorChangeCallback);

//        InitClusterInfoLabel();
//        var rightField = m_UIDocument.rootVisualElement.Q<VisualElement>("Right");
//        m_MenuSelector = new MenuSelector(m_UIDocument, "MenuSelector");

//        var foundRoot = Clam.FFI.NativeMethods.GetRootData(out var dataWrapper);
//        if (foundRoot == FFIError.Ok)
//        {
//            m_IntInputFields = new Dictionary<string, IntTextField>
//        {
//            { "Depth", new IntTextField("Depth", m_UIDocument, 0, Clam.FFI.NativeMethods.TreeHeight(), new Func<bool>(InputFieldChangeCallback)) },
//            { "Cardinality", new IntTextField("Cardinality", m_UIDocument, 0, dataWrapper.Data.cardinality, new Func<bool>(InputFieldChangeCallback)) },
//        };
//        }
//        else
//        {
//            Debug.LogError("root not found");
//        }
//    }

//    void ColorChangeCallback(ChangeEvent<int> changeEvent)
//    {
//        if (changeEvent != null)
//        {
//            if (changeEvent.newValue == 0)
//            {
//                NativeMethods.ColorClustersByEntropy(ColorFiller);
//            }
//        }
//    }

//    unsafe void ColorFiller(ref Clam.FFI.ClusterData nodeData)
//    {
//        GameObject node;

//        bool hasValue = Cakes.Tree.GetTree().TryGetValue(nodeData.ID_AsString(), out node);
//        if (hasValue)
//        {
//            node.GetComponent<Node>().Deselect();
//            node.GetComponent<Node>().SetActualColor(nodeData.color.AsColor);
//        }
//        else
//        {
//            Debug.Log("cluster key not found - color filler - " + nodeData.ID_AsString());
//        }
//    }

//    bool InputFieldChangeCallback()
//    {
//        foreach (var item in Cakes.Tree.GetTree().ToList())
//        {
//            var cluster = item.Value;
//            var wrapper = new RustResourceWrapper<ClusterData>(ClusterData.Alloc(cluster.GetComponent<Node>().GetId()));

//            if (wrapper.result == FFIError.Ok)
//            {
//                if (m_IntInputFields.TryGetValue("Depth", out var depthField))
//                {
//                    if (!depthField.IsWithinRange(wrapper.Data.depth))
//                    {
//                        cluster.GetComponent<Node>().Deselect();
//                        continue;
//                    }
//                }

//                if (m_IntInputFields.TryGetValue("Cardinality", out var cardField))
//                {
//                    if (!cardField.IsWithinRange(wrapper.Data.cardinality))
//                    {
//                        cluster.GetComponent<Node>().Deselect();
//                        continue;
//                    }
//                }
//            }
//            cluster.GetComponent<Node>().Select();
//        }
//        return true;
//    }

//    public void IncludeHiddenInSelection()
//    {
//        foreach (var item in Cakes.Tree.GetTree().ToList())
//        {
//            var cluster = item.Value;
//            var wrapper = new RustResourceWrapper<ClusterData>(ClusterData.Alloc(cluster.GetComponent<Node>().GetId()));
//            if (wrapper != null)
//            {
//                if (m_IntInputFields.TryGetValue("Depth", out var depthField))
//                {
//                    if (!depthField.IsWithinRange(wrapper.Data.depth))
//                    {
//                        cluster.GetComponent<Node>().Deselect();
//                        continue;
//                    }
//                }
//                if (m_IntInputFields.TryGetValue("Cardinality", out var cardField))
//                {
//                    if (!cardField.IsWithinRange(wrapper.Data.cardinality))
//                    {
//                        cluster.GetComponent<Node>().Deselect();
//                        continue;
//                    }
//                }
//                cluster.SetActive(true);
//                cluster.GetComponent<Node>().Select();
//            }
//        }
//    }

//    public void Lock()
//    {
//        m_IntInputFields.ToList().ForEach(item => item.Value.Lock());
//        m_MenuSelector.Lock();
//        var graphMenu = m_UIDocument.rootVisualElement.Q<VisualElement>("GraphMenuInstance");
//        if (graphMenu != null)
//        {
//            graphMenu.Children().ToList().ForEach(c => c.focusable = false);
//        }
//    }

//    public void UnLock()
//    {
//        m_IntInputFields.ToList().ForEach(item => item.Value.UnLock());
//        m_MenuSelector.Unlock();

//        var graphMenu = m_UIDocument.rootVisualElement.Q<VisualElement>("GraphMenuInstance");
//        if (graphMenu != null)
//        {
//            graphMenu.Children().ToList().ForEach(c => c.focusable = true);
//        }
//    }

//    public void DisplayClusterInfo(Clam.FFI.ClusterData data)
//    {
//        if (m_ClusterInfo != null)
//            m_ClusterInfo.text = data.GetInfoForUI();
//    }

//    public void ClearClusterInfo()
//    {
//        if (m_ClusterInfo != null)
//            m_ClusterInfo.text = "";
//    }

//    public void InitClusterInfoLabel()
//    {
//        if (m_ClusterInfo != null)
//        {
//            StringBuilder stringBuilder = new StringBuilder();
//            stringBuilder.AppendLine("id: ");
//            stringBuilder.AppendLine("depth: ");
//            stringBuilder.AppendLine("card: ");
//            stringBuilder.AppendLine("radius: ");
//            stringBuilder.AppendLine("lfd: ");
//            stringBuilder.AppendLine("argC: ");
//            stringBuilder.AppendLine("argR: ");

//            m_ClusterInfoLabel.text = stringBuilder.ToString();
//        }
//    }

//    public void SetSelectedClusterInfo(string value)
//    {
//        if (m_ClusterInfo != null)

//            m_ClusterInfo.text = value;
//    }
//}
