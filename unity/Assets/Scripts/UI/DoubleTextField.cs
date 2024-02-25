using Clam;
using System;
using UnityEngine;
using UnityEngine.EventSystems;
using UnityEngine.InputSystem;
using UnityEngine.UIElements;
using static UnityEngine.Rendering.DebugUI;


class DoubleTextField
{
    Label m_Label;
    TextField m_MinField;
    TextField m_MaxField;
    double m_MinValueThreshold;
    double m_MaxValueThreshold;

    public DoubleTextField(string name, UIDocument document, double minValue, double maxValue)
    {
        m_MinValueThreshold = minValue;
        m_MaxValueThreshold = maxValue;

        m_Label = document.rootVisualElement.Q<Label>(name + "Label");
        m_MinField = document.rootVisualElement.Q<TextField>(name + "Min");
        m_MaxField = document.rootVisualElement.Q<TextField>(name + "Max");

        m_MinField.value = minValue.ToString();
        m_MaxField.value = maxValue.ToString();
        m_Label.focusable = false;
        m_MinField.focusable = false;
        m_MaxField.focusable = false;

        m_MaxField.RegisterValueChangedCallback(MaxFieldCallback);
        m_MinField.RegisterValueChangedCallback(MaxFieldCallback);

        m_MinField.tripleClickSelectsLine = true;
        m_MinField.doubleClickSelectsWord = true;
        m_MaxField.tripleClickSelectsLine = true;
        m_MaxField.doubleClickSelectsWord = true;
    }

    public DoubleTextField(string name, VisualElement parent, double minValue, double maxValue)
    {
        m_MinValueThreshold = minValue;
        m_MaxValueThreshold = maxValue;

        var template = Resources.Load<VisualTreeAsset>("ui/SafeInputFieldTemplate");

        var instance = template.Instantiate();
        parent.Add(instance);
        m_Label = instance.Q<Label>("DataLabel");
        m_MinField = instance.Q<TextField>("MinField");
        m_MaxField = instance.Q<TextField>("MaxField");

        m_Label.text = name;

        m_MinField.value = minValue.ToString();
        m_MaxField.value = maxValue.ToString();
        m_Label.focusable = false;
        m_MinField.focusable = false;
        m_MaxField.focusable = false;

        m_MaxField.RegisterValueChangedCallback(MaxFieldCallback);
        m_MinField.RegisterValueChangedCallback(MinFieldCallback);

        m_MinField.tripleClickSelectsLine = true;
        m_MinField.doubleClickSelectsWord = true;
        m_MaxField.doubleClickSelectsWord = true;
        m_MaxField.doubleClickSelectsWord = true;
    }

    bool ValidateMinNumericInput(ChangeEvent<string> changeEvent)
    {
        if (!ValidateFloatString(changeEvent.newValue, "0123456789."))
        {
            return false;
        }
        else
        {
            double minValue = (double)(object)m_MinValueThreshold;
            double maxValue = (double)(object)m_MaxValueThreshold;
            double curMax = double.Parse(m_MaxField.value);
            double value = double.Parse(changeEvent.newValue);

            if (value < minValue || value > maxValue || value > curMax)
            {
                return false;
            }
            return true;
        }

    }
    void MinFieldCallback(ChangeEvent<string> changeEvent)
    {
        var textField = changeEvent.target as TextField;

        if (!ValidateMinNumericInput(changeEvent))
        {
            textField.value = changeEvent.previousValue;
        }
    }

    void MaxFieldCallback(ChangeEvent<string> changeEvent)
    {
        var textField = changeEvent.target as TextField;

        if (!MaxValueValidation(changeEvent))
        {
            textField.value = changeEvent.previousValue;
        }
    }

    bool MaxValueValidation(ChangeEvent<string> changeEvent)
    {
        if (!ValidateFloatString(changeEvent.newValue, "0123456789."))
        {
            return false;
        }
        else
        {
            double minValue = (double)(object)m_MinValueThreshold;
            double maxValue = (double)(object)m_MaxValueThreshold;
            double curMin = double.Parse(m_MinField.value);
            double value = double.Parse(changeEvent.newValue);

            if (value < minValue || value > maxValue || value < curMin)
            {
                return false;
            }
            return true;
        }
    }
    bool ValidateFloatString(string value, string validCharacters)
    {
        foreach (var c in value)
        {
            if ((c < '0' || c > '9') && c != '.')
            {
                return false;
            }
        }

        return true;
    }

    public void Lock()
    {
        m_MinField.focusable = m_MaxField.focusable = false;
        m_MaxField.isReadOnly = m_MinField.isReadOnly = true;
    }

    public void UnLock()
    {
        m_MinField.focusable = m_MaxField.focusable = true;
        m_MaxField.isReadOnly = m_MinField.isReadOnly = false;
    }

    public Vector2Int MinMaxInt()
    {
        return new Vector2Int(int.Parse(m_MinField.value), int.Parse(m_MaxField.value));
    }

    public Tuple<double, double> MinMaxRange()
    {
        return new Tuple<double, double>(double.Parse(m_MinField.value), double.Parse(m_MaxField.value));
    }
}
