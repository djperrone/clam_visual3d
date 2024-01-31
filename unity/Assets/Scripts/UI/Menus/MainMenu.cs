using Clam;
using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UIElements;

public class MainMenu : MonoBehaviour
{
    UIDocument m_UIDocument;
    Button m_CreateNewButton;
    Button m_LoadButton;
    Button m_ExitButton;


    // Start is called before the first frame update
    void Start()
    {
        m_UIDocument = GetComponent<UIDocument>();

        m_CreateNewButton = m_UIDocument.rootVisualElement.Q<Button>("CreateTree");
        m_LoadButton = m_UIDocument.rootVisualElement.Q<Button>("LoadTree");
        m_ExitButton = m_UIDocument.rootVisualElement.Q<Button>("ExitButton");

        SetExitButtonCallback(m_ExitButton);
        SetCreateButtonCallback();
        m_LoadButton.RegisterCallback<ClickEvent>(LoadButtonCallback);
    }

    private void SetCreateButtonCallback()
    {
        m_CreateNewButton.RegisterCallback<ClickEvent>(CreateButtonCallback);
    }

    private void CreateButtonCallback(ClickEvent evt)
    {
        Button button = evt.currentTarget as Button;
        Clam.MenuEventManager.SwitchState(Menu.CreateNewTree);

    }
    private void LoadButtonCallback(ClickEvent evt)
    {
        Button button = evt.currentTarget as Button;
        Clam.MenuEventManager.SwitchState(Menu.LoadTree);
    }

    private void SetExitButtonCallback(Button button)
    {
        button.RegisterCallback<ClickEvent>(ExitButtonCallback);
    }

    private void ExitButtonCallback(ClickEvent evt)
    {
        Button button = evt.currentTarget as Button;
        evt.StopImmediatePropagation();

        var template = Resources.Load<VisualTreeAsset>("ui/AreYouSure");
        var instance = template.Instantiate();

        UIHelpers.ShowPopup(m_UIDocument.rootVisualElement, instance);

        var overlay = m_UIDocument.rootVisualElement.Q<VisualElement>("Overlay");
        overlay.style.backgroundColor = new StyleColor(new Color(0, 0, 0, 0.71f));
        var yesButton = m_UIDocument.rootVisualElement.Q<Button>("PopUpYesButton");
        var noButton = m_UIDocument.rootVisualElement.Q<Button>("PopUpNoButton");

        noButton.clickable.clicked += () =>
        {
            UIHelpers.PopupClose(m_UIDocument.rootVisualElement, m_UIDocument.rootVisualElement.Q<VisualElement>("PopUpElement"));
        };

        yesButton.clickable.clicked += () =>
        {
            MenuEventManager.instance.Quit();
        };

    }

    // Update is called once per frame
    void Update()
    {

    }
}
