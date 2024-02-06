using Clam;
using Clam.FFI;
using StarterAssets;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;
using UnityEngine.InputSystem;
using UnityEngine.UIElements;

public class SideMenu : MonoBehaviour
{
    public List<VisualTreeAsset> m_TreeAssets;
    UIDocument m_UIDocument;

    string m_CurrentMenuName = "TreeMenu";

    DropdownField m_DropdownField;

    ClusterMenu m_ClusterMenu;
    GraphBuildMenu m_GraphBuildMenu;
    ClamGraphBuildMenu m_ClamGraphBuildMenu;
    TreeMenu m_TreeMenu;

    RadioButtonGroup m_UIMode;

    // Start is called before the first frame update
    void Start()
    {
        m_UIDocument = GetComponent<UIDocument>();
        m_DropdownField = m_UIDocument.rootVisualElement.Q<DropdownField>("MenuSelector");
        m_DropdownField.focusable = false;

        m_DropdownField.choices = new List<string>()
        {
            "TreeMenu", "ClusterMenu", "Manually Create Graph (beta)", "CHAODA Graph"
        };
        m_DropdownField.value = m_CurrentMenuName;

        var rightField = m_UIDocument.rootVisualElement.Q<VisualElement>("Right");

        for (int i = 0; i < m_TreeAssets.Count; i++)
        {
            var asset = m_TreeAssets[i];
            var instance = asset.Instantiate();
            if (i != 0)
            {
                instance.style.display = DisplayStyle.None;
            }

            instance.name = m_DropdownField.choices[i];
            rightField.hierarchy.Insert(i + 1, instance);
            //rightField.hierarchy.Add(instance);
        }
        m_DropdownField.RegisterValueChangedCallback(Callback);

        m_TreeMenu = new TreeMenu(m_UIDocument);
        m_ClusterMenu = new ClusterMenu(GetComponent<UIDocument>());
        m_GraphBuildMenu = new GraphBuildMenu(GetComponent<UIDocument>(), "GraphBuildMenu");
        m_ClamGraphBuildMenu = new ClamGraphBuildMenu(GetComponent<UIDocument>(), "ClamGraphBuildMenu");
        m_UIMode = m_UIDocument.rootVisualElement.Q<RadioButtonGroup>("UIMode");
        m_UIMode.choices = new List<string>() { "In-World", "Overlay" };
        m_UIMode.RegisterValueChangedCallback(UIModeCallback);

    }
    void UIModeCallback(ChangeEvent<int> changeEvent)
    {
        if (changeEvent.newValue == 0)
        {
            ClamUserInput input = (ClamUserInput)FindObjectOfType(typeof(ClamUserInput));
            input.GetComponent<ClamUserInput>().OnChangeMapToPlayer(new InputValue());
        }
        else
        {
            FirstPersonController input = (FirstPersonController)FindObjectOfType(typeof(FirstPersonController));
            input.GetComponent<FirstPersonController>().OnChangeMapToWorldUI(new InputValue());
        }
    }
    public void SwitchToOverlayUIMode()
    {
        m_UIMode.value = 1;
    }

    public void SwitchToCameraControlMode()
    {
        m_UIMode.value = 0;
    }

    public void DisplayClusterInfo(ClusterData data)
    {
        m_ClusterMenu.DisplayClusterInfo(data);
    }
    public void ClearClusterInfo()
    {
        m_ClusterMenu.ClearClusterInfo();
    }

    void Callback(ChangeEvent<string> evt)
    {
        var rightFieldChildrenList = m_UIDocument.rootVisualElement.Q<VisualElement>("Right").Children().ToList();

        var currentMenu = rightFieldChildrenList.Find(x => x.name == m_CurrentMenuName);
        if (currentMenu != null)
        {
            currentMenu.style.display = DisplayStyle.None;
        }
        else
        {
            Debug.LogError("couldnt find  current menu");
        }

        var nextMenu = rightFieldChildrenList.Find(x => x.name == evt.newValue);
        if (nextMenu != null)
        {
            nextMenu.style.display = DisplayStyle.Flex;
            m_CurrentMenuName = nextMenu.name;
        }
        else
        {
            Debug.LogError("couldnt find next menu");
        }
    }

    public void SetFocusable(bool focusable)
    {
        var rightField = m_UIDocument.rootVisualElement.Q<VisualElement>("Right");

        if (rightField != null)
        {
            SetFocusableRecursive(rightField, focusable);
        }
    }

    void SetFocusableRecursive(VisualElement element, bool focusable)
    {
        element.Children().ToList().ForEach(c =>
        {
            c.focusable = focusable;
            SetFocusableRecursive(c, focusable);
        });
    }
}
