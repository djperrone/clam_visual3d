using Clam.FFI;
using Clam;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UIElements;
using System;
using System.Linq;

public class AccuracyTestMenu
{
    RadioButtonGroup m_ColorOptions;
    private UIDocument m_UIDocument;

    Button m_Run;

  

    public AccuracyTestMenu(UIDocument uidocument)
    {
        m_UIDocument = uidocument;
        m_ColorOptions = m_UIDocument.rootVisualElement.Q<RadioButtonGroup>("TreeColorOptions");
        m_ColorOptions.choices = new List<string>() { "Label", "Cardinality", "Radius", "LFD", "Depth", "VertexDegree" };//, "Component", "Ratios" };


        m_Run.RegisterCallback<ClickEvent>(RunCallback);

        m_Run = m_UIDocument.rootVisualElement.Q<Button>("RunPhysicsAccuracyTest");



    }

    void RunCallback(ClickEvent evt)
    {
    }

   
}
