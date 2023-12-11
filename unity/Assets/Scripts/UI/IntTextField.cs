using Clam;
using Clam.FFI;
using System;
using System.Collections.Generic;
using System.Runtime.CompilerServices;
using System.Text;
using System.Xml.Linq;
using UnityEditor;
using UnityEngine;
using UnityEngine.EventSystems;
using UnityEngine.InputSystem;
using UnityEngine.UIElements;
using static UnityEngine.Rendering.DebugUI;

namespace Clam
{

    class IntTextField
    {
        Label m_Label;
        TextField m_MinField;
        TextField m_MaxField;
        MinMaxSlider m_Slider;
        int m_MinValueThreshold;
        int m_MaxValueThreshold;
        Func<bool> m_Callback;

        public IntTextField(string name, UIDocument document, int minValue, int maxValue, Func<bool> callback)
        {

            m_MinValueThreshold = minValue;
            m_MaxValueThreshold = maxValue;

            m_Callback = callback;

            m_Label = document.rootVisualElement.Q<Label>(name + "Label");
            m_MinField = document.rootVisualElement.Q<TextField>(name + "Min");
            m_MaxField = document.rootVisualElement.Q<TextField>(name + "Max");
            m_Slider = document.rootVisualElement.Q<MinMaxSlider>(name + "Slider");

            m_MinField.value = minValue.ToString();
            m_MaxField.value = maxValue.ToString();
            m_Slider.highLimit = m_Slider.maxValue = maxValue;
            m_Slider.lowLimit = m_Slider.minValue = minValue;

            m_Label.focusable = false;
            m_MinField.focusable = false;
            m_MaxField.focusable = false;
            m_Slider.focusable = false;

            m_MaxField.RegisterValueChangedCallback(MaxFieldCallback);
            m_MinField.RegisterValueChangedCallback(MinFieldCallback);
            m_Slider.RegisterValueChangedCallback(SliderCallback);

            m_MinField.tripleClickSelectsLine = true;
            m_MinField.doubleClickSelectsWord = true;
            m_MaxField.tripleClickSelectsLine = true;
            m_MaxField.doubleClickSelectsWord = true;
        }

        public IntTextField(string name, VisualElement parent, int minValue, int maxValue, Func<bool> callback)
        {
            var template = Resources.Load<VisualTreeAsset>("ui/SafeInputFieldTemplate");

            var instance = template.Instantiate();
            parent.Add(instance);
            m_MinValueThreshold = minValue;
            m_MaxValueThreshold = maxValue;

            m_Callback = callback;

            m_MinField.value = minValue.ToString();
            m_MaxField.value = maxValue.ToString();
            m_Slider.highLimit = m_Slider.maxValue = maxValue;
            m_Slider.lowLimit = m_Slider.minValue = minValue;

            m_Label.focusable = false;
            m_MinField.focusable = false;
            m_MaxField.focusable = false;

            m_MaxField.RegisterValueChangedCallback(MaxFieldCallback);
            m_MinField.RegisterValueChangedCallback(MinFieldCallback);

            m_Slider.RegisterValueChangedCallback(SliderCallback);

            m_MinField.tripleClickSelectsLine = true;
            m_MinField.doubleClickSelectsWord = true;
            m_MaxField.tripleClickSelectsLine = true;
            m_MaxField.doubleClickSelectsWord = true;
        }

        void SliderCallback(ChangeEvent<Vector2> evt)
        {
            var slider = evt.target as MinMaxSlider;
            m_Callback();
        }

        bool ValidateMinNumericInput(ChangeEvent<string> changeEvent)
        {
            if (!ValidateCharacters(changeEvent.newValue, "0123456789."))
            {
                return false;

            }
            else
            {
                if (changeEvent.newValue.Contains('.'))
                {
                    return false;
                }
                int minValue = (int)(object)m_MinValueThreshold;
                int maxValue = (int)(object)m_MaxValueThreshold;
                int curMax = int.Parse(m_MaxField.value);
                int value = int.Parse(changeEvent.newValue);

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
            else
            {
                m_Slider.minValue = int.Parse(textField.value);
                m_Callback();
            }
        }

        void MaxFieldCallback(ChangeEvent<string> changeEvent)
        {
            var textField = changeEvent.target as TextField;

            if (!MaxValueValidation(changeEvent))
            {
                textField.value = changeEvent.previousValue;
            }
            else
            {
                m_Slider.maxValue = int.Parse(textField.value);
                m_Callback();
            }
        }

        bool MaxValueValidation(ChangeEvent<string> changeEvent)
        {
            if (!ValidateCharacters(changeEvent.newValue, "0123456789."))
            {
                return false;
            }
            else
            {
                if (changeEvent.newValue.Contains('.'))
                {
                    return false;
                }
                int minValue = (int)(object)m_MinValueThreshold;
                int maxValue = (int)(object)m_MaxValueThreshold;
                int curMin = int.Parse(m_MinField.value);
                int value = int.Parse(changeEvent.newValue);

                if (value < minValue || value > maxValue || value < curMin)
                {
                    return false;
                }
                return true;
            }
        }
        bool ValidateCharacters(string value, string validCharacters)
        {
            foreach (var c in value)
            {
                if ((c < '0' || c > '9'))
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

        public Tuple<int, int> MinMaxRange()
        {
            return new Tuple<int, int>(int.Parse(m_MinField.value), int.Parse(m_MaxField.value));
        }

        public bool IsWithinRange(FFI.RustResourceWrapper<ClusterData> wrapper)
        {
            List<Tuple<string, int>> comparisons = new List<Tuple<string, int>>();
            comparisons.Add(new Tuple<string, int>("Depth", wrapper.GetData().depth));
            comparisons.Add(new Tuple<string, int>("Cardinality", wrapper.GetData().cardinality));
            comparisons.Add(new Tuple<string, int>("ArgRadius", wrapper.GetData().argRadial));
            comparisons.Add(new Tuple<string, int>("ArgCenter", wrapper.GetData().argCenter));

            foreach ((string name, int value) in comparisons)
            {
                if (m_Label.text == name)
                {
                    (int min, int max) = MinMaxRange();
                    if (value < min || value > max)
                    {
                        return false;
                    }
                }
            }

            return true;
        }

        public bool IsWithinRange(int value)
        {
            (int min, int max) = MinMaxRange();
            return (value >= min && value <= max);
        }
    }
}
