using Clam;
using System;
using System.Collections;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using UnityEngine;
using UnityEngine.UIElements;

public class LoadTree : MonoBehaviour
{
    UIDocument m_UIDocument;

    TextField m_LoadTreeField;
    DropdownField m_LoadTreeDropdownField;

    Button m_LoadButton;
    Button m_BackButton;

    string m_DataDirectory = "../data/binaries/";

    // Start is called before the first frame update
    void Start()
    {
        m_UIDocument = GetComponent<UIDocument>();

        m_LoadTreeField = m_UIDocument.rootVisualElement.Q<TextField>("LoadTreeInputField");
        m_BackButton = m_UIDocument.rootVisualElement.Q<Button>("BackButton");
        m_LoadButton = m_UIDocument.rootVisualElement.Q<Button>("LoadButton");
        m_BackButton.RegisterCallback<ClickEvent>(BackButtonCallback);
        m_LoadButton.RegisterCallback<ClickEvent>(LoadButtonCallback);

        m_LoadTreeDropdownField = m_UIDocument.rootVisualElement.Q<DropdownField>("LoadTreeDropdown");

        var dirNames = Directory.GetDirectories(m_DataDirectory);
        var extractedDirNames = dirNames.Select(dirName => Path.GetFileName(dirName));

        m_LoadTreeDropdownField.choices = extractedDirNames.ToList();

        m_LoadTreeDropdownField.RegisterValueChangedCallback(evt =>
        {
            m_LoadTreeField.value = evt.newValue;
        });
    }

    void BackButtonCallback(ClickEvent evt)
    {
        MenuEventManager.SwitchState(Menu.Main);
    }

    void LoadButtonCallback(ClickEvent evt)
    {
        var validNames = Directory.GetDirectories(m_DataDirectory);
        string dataName = m_LoadTreeField.text;
        dataName = "../data/binaries/" + dataName;

        if (validNames.Contains(dataName))
        {
            Clam.MenuEventManager.SwitchState(Menu.LoadClam);
        }
        else
        {
            Debug.LogError("Path not found: " + dataName);
            ErrorDialoguePopup();
        }
    }

    void ErrorDialoguePopup()
    {
        var template = Resources.Load<VisualTreeAsset>("ui/InvalidInputPopup");
        var instance = template.Instantiate();
        m_UIDocument.rootVisualElement.Add(instance);

        UIHelpers.ShowPopup(m_UIDocument.rootVisualElement, instance);

        var overlay = m_UIDocument.rootVisualElement.Q<VisualElement>("Overlay");
        overlay.style.backgroundColor = new StyleColor(new Color(0, 0, 0, 0.71f));
        var okButton = m_UIDocument.rootVisualElement.Q<Button>("PopUpOkButton");

        okButton.clickable.clicked += () =>
        {
            UIHelpers.PopupClose(m_UIDocument.rootVisualElement, m_UIDocument.rootVisualElement.Q<VisualElement>("PopUpElement"));
        };
    }

    // Update is called once per frame
    void Update()
    {

    }


}
