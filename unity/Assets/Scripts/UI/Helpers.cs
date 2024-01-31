
using Clam;
using System;
using UnityEngine;
using UnityEngine.UIElements;

public static class UIHelpers
{
    public static void ShowErrorPopUP(string errorMessage)
    {
        var template = Resources.Load<VisualTreeAsset>("ui/InvalidInputPopup");
        var instance = template.Instantiate();
        var uiDoc = MenuEventManager.instance.GetCurrentMenu().GetComponent<UIDocument>();
        
        uiDoc.rootVisualElement.Add(instance);
        var label = uiDoc.rootVisualElement.Q<Label>("InvalidInputLabel");
        label.text = errorMessage;


        UIHelpers.ShowPopup(uiDoc.rootVisualElement, instance);

        var overlay = uiDoc.rootVisualElement.Q<VisualElement>("Overlay");
        overlay.style.backgroundColor = new StyleColor(new Color(0, 0, 0, 0.71f));
        var okButton = uiDoc.rootVisualElement.Q<Button>("PopUpOkButton");

        okButton.clickable.clicked += () =>
        {
            UIHelpers.PopupClose(uiDoc.rootVisualElement, uiDoc.rootVisualElement.Q<VisualElement>("PopUpElement"));
        };
    }

    public static void ShowPopup(VisualElement rootElementForPopup,
        VisualElement popupContent,
        float widthInPercents = 100.0f,
        float heightInPercents = 100.0f)
    {
        if (widthInPercents <= 0f || widthInPercents > 100f)
        {
            throw new ArgumentException($"Width should be in the range 0 < width < 100.", "widthInPercents");
        }

        if (heightInPercents <= 0f || heightInPercents > 100f)
        {
            throw new ArgumentException($"Height should be in the range 0 < height < 100.", "heightInPercents");
        }

        //Create visual element for popup
        var popupElement = new VisualElement();
        popupElement.name = "PopUpElement";
        popupElement.style.position = new StyleEnum<Position>(Position.Absolute);
        popupElement.style.top = 0;
        popupElement.style.left = 0;
        popupElement.style.flexGrow = new StyleFloat(1);
        popupElement.style.height = new StyleLength(new Length(100, LengthUnit.Percent));
        popupElement.style.width = new StyleLength(new Length(100, LengthUnit.Percent));

       
        //Set content size
        popupContent.style.width = new StyleLength(new Length(widthInPercents, LengthUnit.Percent));
        popupContent.style.height = new StyleLength(new Length(heightInPercents, LengthUnit.Percent));

        //Show popupContent in the middle of the screen
        popupContent.style.position = new StyleEnum<Position>(Position.Absolute);

        float topAndBottom = (100f - heightInPercents) / 2f;
        popupContent.style.top = new StyleLength(new Length(topAndBottom, LengthUnit.Percent));
        popupContent.style.bottom = new StyleLength(new Length(topAndBottom, LengthUnit.Percent));

        float leftAndRight = (100f - widthInPercents) / 2f;
        popupContent.style.left = new StyleLength(new Length(leftAndRight, LengthUnit.Percent));
        popupContent.style.right = new StyleLength(new Length(leftAndRight, LengthUnit.Percent));

        popupElement.Add(popupContent);

        rootElementForPopup.Add(popupElement);
    }

    public static void PopupClose(VisualElement popupRoot, VisualElement popup)//,
    {
        popupRoot.Remove(popup);
    }
}
