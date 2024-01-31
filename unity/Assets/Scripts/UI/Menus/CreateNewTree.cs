using Clam;
using System;
using System.Collections;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using UnityEngine;
using UnityEngine.UIElements;

public class CreateNewTree : MonoBehaviour
{
    UIDocument m_UIDocument;

    TextField m_DatasetField;
    TextField m_CardinalityField;
    DropdownField m_DatasetDropdownField;
    DropdownField m_DistanceMetricDropdownField;

    Button m_CreateButton;
    Button m_BackButton;

    string m_DataDirectory = "../data/anomaly_data/preprocessed";


    // Start is called before the first frame update
    void Start()
    {
        m_UIDocument = GetComponent<UIDocument>();

        m_DatasetField = m_UIDocument.rootVisualElement.Q<TextField>("DatasetInputField");
        m_CardinalityField = m_UIDocument.rootVisualElement.Q<TextField>("CardinalityInputField");
        m_BackButton = m_UIDocument.rootVisualElement.Q<Button>("BackButton");
        m_CreateButton = m_UIDocument.rootVisualElement.Q<Button>("CreateButton");
        m_BackButton.RegisterCallback<ClickEvent>(BackButtonCallback);
        m_CreateButton.RegisterCallback<ClickEvent>(CreateButtonCallback);

        m_DatasetDropdownField = m_UIDocument.rootVisualElement.Q<DropdownField>("DatasetDropdown");
        m_DistanceMetricDropdownField = m_UIDocument.rootVisualElement.Q<DropdownField>("DistanceMetricDropdown");

        var fileNames = GetAnomalyFiles(m_DataDirectory);

        m_DatasetDropdownField.choices = fileNames.ToList();

        m_DatasetDropdownField.RegisterValueChangedCallback(evt =>
        {
            m_DatasetField.value = evt.newValue;
        });

        m_DistanceMetricDropdownField.choices = new List<string> {
        "Euclidean",
        "EuclideanSQ",
        "Manhattan",
        "L3Norm",
        "L4Norm",
        "Chebyshev",
        "Cosine",
        "Canberra" };
        //"NeedlemanWunsch",
        //"Levenshtein" };
    }

    void DropdownCallback(EventCallback<ChangeEvent<string>> evt)
    {

    }
    void BackButtonCallback(ClickEvent evt)
    {
        MenuEventManager.SwitchState(Menu.Main);
    }
    void CreateButtonCallback(ClickEvent evt)
    {
        var validNames = GetAnomalyFiles(m_DataDirectory);

        string dataName = m_DatasetField.text;

        if (uint.TryParse(m_CardinalityField.text, out uint cardinality) && validNames.Contains(dataName) && m_DistanceMetricDropdownField.index >= 0)
        {
            Clam.MenuEventManager.SwitchState(Menu.StartClam);
        }
        else
        {
            ErrorDialoguePopup();
        }
    }

    void LoadButtonCallback(ClickEvent evt)
    {
        string loadDir = "../data/binaries";
        var validNames = Directory.GetFiles(loadDir);
        string dataName = m_DatasetField.text;

        if (uint.TryParse(m_CardinalityField.text, out uint cardinality) && validNames.Contains(dataName))
        {
            Clam.MenuEventManager.SwitchState(Menu.StartClam);
        }
        else
        {
            ErrorDialoguePopup();
        }
    }

    private HashSet<string> GetBinaryFiles(string dir)
    {
        HashSet<string> fileNames = new HashSet<string>();
        try
        {
            foreach (string f in Directory.GetFiles(dir))
            {
                var name = GetUntilOrEmpty(Path.GetFileName(f), "_");

                fileNames.Add(name);
            }

            return fileNames;
        }
        catch (System.Exception ex)
        {
            Debug.Log(ex.Message);
        }
        return new HashSet<string>();

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

    private HashSet<string> GetAnomalyFiles(string dir)
    {
        HashSet<string> fileNames = new HashSet<string>();
        try
        {
            foreach (string f in Directory.GetFiles(dir))
            {
                var name = GetUntilOrEmpty(Path.GetFileName(f), "_");
                fileNames.Add(name);
            }
            return fileNames;
        }
        catch (System.Exception ex)
        {
            Debug.Log(ex.Message);
        }
        return new HashSet<string>();

    }

    private string GetUntilOrEmpty(string text, string stopAt = "-")
    {
        if (!String.IsNullOrWhiteSpace(text))
        {
            int charLocation = text.IndexOf(stopAt, StringComparison.Ordinal);

            if (charLocation > 0)
            {
                return text.Substring(0, charLocation);
            }
        }

        return String.Empty;
    }


    // Update is called once per frame
    void Update()
    {

    }


}
