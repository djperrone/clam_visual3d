

using System.Collections.Generic;
using UnityEngine.UIElements;
using UnityEngine;
using System.Linq;

public class MenuSelector
{
    DropdownField m_DropdownField;
    UIDocument m_Document;
    GraphBuildMenu m_GraphBuildMenu;

    public MenuSelector(UIDocument document, string name)//, GameObject graphBuilderPrefab)
    {
        m_DropdownField = document.rootVisualElement.Q<DropdownField>(name);
        m_Document = document;

        m_DropdownField.focusable = false;

        m_DropdownField.choices = new List<string>()
        {
            "Cluster Details", "Graph Builder", "Tree Settings"
        };

        m_DropdownField.RegisterValueChangedCallback(Callback);

        m_GraphBuildMenu = null;
    }

    void Callback(ChangeEvent<string> evt)
    {
        if (evt.newValue == m_DropdownField.choices[0])
        {
            var rightField = m_Document.rootVisualElement.Q<VisualElement>("Right");

            var children = rightField.Children();
            var graphMenu = children.ToList().Find(x => x.name == "GraphBuildMenuInstance");
            if (graphMenu != null)
            {
                rightField.Remove(graphMenu);
            }
            else
            {
                Debug.Log("cant find graph menu");
            }
            var clusterDetailers = children.ToList().Find(x => x.name == "ClusterInfo");
            clusterDetailers.style.display = DisplayStyle.Flex;
        }
        else if (evt.newValue == m_DropdownField.choices[1])
        {
            var rightField = m_Document.rootVisualElement.Q<VisualElement>("Right");

            if (rightField != null)
            {
                var children = rightField.Children();
                var clusterDetailersIndex = children.ToList().FindIndex(x => x.name == "ClusterInfo");
                var clusterDetailers = children.ToList().Find(x => x.name == "ClusterInfo");

                if (clusterDetailersIndex != -1)
                {
                    clusterDetailers.style.display = DisplayStyle.None;

                    var template = Resources.Load<VisualTreeAsset>("ui/GraphBuildMenu");
                    var instance = template.Instantiate();
                    instance.name = "GraphBuildMenuInstance";

                    rightField.Add(instance);
                    if (m_GraphBuildMenu == null)
                    {
                        m_GraphBuildMenu = new GraphBuildMenu(m_Document, "GraphBuildMenuInstance");//, m_GraphBuilderPrefab);
                    }
                }
            }

        }
        else
        {
            Debug.Log("invalid selection...somehow?");
        }
    }

    public void Lock()
    {
        m_DropdownField.focusable = false;
    }
    public void Unlock()
    {
        m_DropdownField.focusable = true;
    }
}
