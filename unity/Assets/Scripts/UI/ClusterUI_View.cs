using Clam;
using Clam.FFI;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using Unity.VisualScripting;
using UnityEditor;
using UnityEngine;
using UnityEngine.UI;
using UnityEngine.UIElements;
using static UnityEngine.Rendering.DebugUI;

public class ClusterUI_View : MonoBehaviour
{
    [SerializeField]
    VisualTreeAsset m_SafeInputFieldTemplate;

    private UIDocument m_UIDocument;
    //private Dictionary<string, GameObject> m_Tree;
    Label m_ClusterInfo;
    Label m_ClusterInfoLabel;
    RadioButtonGroup m_ColorOptions;
    MenuSelector m_MenuSelector;

    //SafeTextField m_DepthField;
    //SafeTextField m_CardinalityField;

    Dictionary<string, IntTextField> m_IntInputFields;
    //Dictionary<string, DoubleTextField> m_DoubleInputFields;

    public VisualTreeAsset m_GraphBuilder;
    public VisualTreeAsset m_TreeSettings;


    //public float radius;
    //public float lfd;
    //public int argCenter;
    //public int argRadius;


    public void Start()
    {
        m_UIDocument = GetComponent<UIDocument>();

        m_ClusterInfo = m_UIDocument.rootVisualElement.Q<Label>("ClusterInfo");
        m_ClusterInfoLabel = m_UIDocument.rootVisualElement.Q<Label>("ClusterInfoLabel");
        m_ColorOptions = m_UIDocument.rootVisualElement.Q<RadioButtonGroup>("ColorOptions");
        m_ColorOptions.RegisterValueChangedCallback(ColorChangeCallback);

        InitClusterInfoLabel();
        var rightField = m_UIDocument.rootVisualElement.Q<VisualElement>("Right");
        m_MenuSelector = new MenuSelector(m_UIDocument, "MenuSelector");
        //m_MenuSelector = m_UIDocument.rootVisualElement.Q<DropdownField>("MenuSelector");

        //var myDelegate = new Func<string, DialogResult>(MessageBox.Show);
        //TryDo.Do(myDelegate, null)

        var foundRoot = Clam.FFI.NativeMethods.GetRootData(out var dataWrapper);
        if (foundRoot == FFIError.Ok)
        {
            m_IntInputFields = new Dictionary<string, IntTextField>
        {
            //{ "Depth", new IntTextField("Depth", m_UIDocument, 0, 10, new Func<bool>(InputFieldChangeCallback)) },
            { "Depth", new IntTextField("Depth", m_UIDocument, 0, Clam.FFI.NativeMethods.TreeHeight(), new Func<bool>(InputFieldChangeCallback)) },
            //{ "Cardinality", new IntTextField("Cardinality", m_UIDocument, 0, 10, new Func<bool>(InputFieldChangeCallback)) },
            { "Cardinality", new IntTextField("Cardinality", m_UIDocument, 0, dataWrapper.Data.cardinality, new Func<bool>(InputFieldChangeCallback)) },

            //{ "ArgRadius", new IntTextField("ArgRadius", rightField, 0, ClamFFI.ArgRadius(), new Func < bool >(InputFieldChangeCallback)) },
            //{ "ArgCenter", new IntTextField("ArgCenter", rightField, 0, ClamFFI.ArgCenter(), new Func < bool >(InputFieldChangeCallback)) }
        };
        }
        else
        {
            Debug.LogError("root not found");
        }



        //m_DoubleInputFields = new Dictionary<string, DoubleTextField>
        //{
        //    { "Radius", new DoubleTextField("Radius", rightField, 0, ClamFFI.Radius()) },
        //    { "lfd", new DoubleTextField("lfd", rightField, 0, ClamFFI.LFD()) }
        //};

        //m_Tree = MenuEventManager.instance.GetTree();

        //InitMenuSelector();

        //var inputField = m_SafeInputFieldTemplate.Instantiate();

        //VisualTreeAsset uiAsset = AssetDatabase.LoadAssetAtPath<VisualTreeAsset>("Assets/ui/SafeInputFieldTemplate.uxml");
        //VisualTreeAsset uiAsset2 = AssetDatabase.LoadAssetAtPath<VisualTreeAsset>("Assets/ui/MyInputField.uxml");k
        //VisualTreeAsset uiAsset3 = AssetDatabase.LoadAssetAtPath<VisualTreeAsset>("Assets/ui/inputTemplate.uxml");

        //VisualElement ui = uiAsset.Instantiate();

        //rightField.Add(ui);
        //rightField.Add(uiAsset2.Instantiate());
        //rightField.Add(uiAsset3.Instantiate());
        //inputField.userData
        //inputField.Q<Label>()
        //rightField.Add(m_InputFieldTemplate.Instantiate());


        //m_DepthField = new SafeTextField("Depth", m_UIDocument, 0, ClamFFI.TreeHeight());
        //m_CardinalityField = new SafeTextField("Cardinality", m_UIDocument, 0, ClamFFI.Cardinality());
    }

    void ColorChangeCallback(ChangeEvent<int> changeEvent)
    {
        if (changeEvent != null)
        {
            if (changeEvent.newValue == 0)
            {
                NativeMethods.ColorClustersByLabel(ColorFiller);
            }
        }
    }

    unsafe void ColorFiller(ref Clam.FFI.ClusterData nodeData)
    {
        GameObject node;

        bool hasValue = Cakes.Tree.GetTree().TryGetValue(nodeData.id.AsString, out node);
        if (hasValue)
        {
            Debug.Log("setting color to" + nodeData.color.AsVector3.ToString());
            node.GetComponent<Node>().Deselect();
            node.GetComponent<Node>().SetActualColor(nodeData.color.AsColor);
        }
        else
        {
            Debug.Log("cluster key not found - color filler - " + nodeData.id);
        }
    }

    bool InputFieldChangeCallback()
    {
        foreach (var item in Cakes.Tree.GetTree().ToList())
        {
            var cluster = item.Value;
            var wrapper = new RustResourceWrapper<ClusterData>(ClusterData.Alloc(cluster.GetComponent<Node>().GetId()));

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


    public void IncludeHiddenInSelection()
    {
        foreach (var item in Cakes.Tree.GetTree().ToList())
        {
            var cluster = item.Value;
            //if (!cluster.activeSelf)
            //{
            //    continue;
            //}

            //Clam.FFI.ClusterDataWrapper wrapper = new Clam.FFI.ClusterDataWrapper(cluster.GetComponent<Node>().ToNodeData());
            //Clam.FFI.NativeMethods.GetClusterData(wrapper);
            var wrapper = new RustResourceWrapper<ClusterData>(ClusterData.Alloc(cluster.GetComponent<Node>().GetId()));
            if (wrapper != null)
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
            }
            {
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
            cluster.SetActive(true);
            cluster.GetComponent<Node>().Select();
        }
    }

    public void Lock()
    {

        m_IntInputFields.ToList().ForEach(item => item.Value.Lock());
        m_MenuSelector.Lock();
        var graphMenu = m_UIDocument.rootVisualElement.Q<VisualElement>("GraphMenuInstance");
        if (graphMenu != null)
        {
            graphMenu.Children().ToList().ForEach(c => c.focusable = false);
        }

    }



    public void UnLock()
    {

        m_IntInputFields.ToList().ForEach(item => item.Value.UnLock());
        m_MenuSelector.Unlock();

        var graphMenu = m_UIDocument.rootVisualElement.Q<VisualElement>("GraphMenuInstance");
        if (graphMenu != null)
        {
            graphMenu.Children().ToList().ForEach(c => c.focusable = true);
        }
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
}
