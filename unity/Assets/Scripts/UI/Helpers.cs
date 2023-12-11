
using System;
using UnityEngine.UIElements;

public static class UIHelpers
{
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
