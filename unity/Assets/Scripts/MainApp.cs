using ClamFFI;
using System.Collections;
using System.Collections.Generic;
using UnityEditor;
using UnityEngine;
using UnityEngine.UIElements;

public class MainApp : MonoBehaviour
{
    public GameObject nodePrefab;
    public GameObject treePrefab;

    [SerializeField]
    VisualTreeAsset m_ClusterScriptTemplate;

    private InputHandler m_InputHandler;
    private ClusterUI_Script m_ClusterUIScript;
    private UIDocument m_UIDocument; 



    // Start is called before the first frame update
    void Start()
    {
        treePrefab.GetComponent<TreeScript>().Init();
        m_InputHandler = new InputHandler();
        print("hello");

        var uiDocument = GetComponent<UIDocument>();
        m_ClusterUIScript = new ClusterUI_Script();
        m_ClusterUIScript.Intialize(uiDocument.rootVisualElement, m_ClusterScriptTemplate);

        m_ClusterUIScript.DepthSlider().RegisterValueChangedCallback<Vector2>((evt) =>
        {
            //Debug.LogFormat(" New slider values: {0} .. {1}", evt.newValue.x, evt.newValue.y);
            //Debug.Log("Depth: (" + ((int)evt.newValue.x).ToString() + ", " + ((int)evt.newValue.y).ToString() + ")");
            m_ClusterUIScript.DepthSlider().label = "Depth: (" + ((int)evt.newValue.x).ToString() + ", " + ((int)evt.newValue.y).ToString() + ")";
            GetTree().SetDepthRange(evt.newValue);
        });
    }

    // Update is called once per frame
    void Update()
    {
        m_InputHandler.Update(m_ClusterUIScript, treePrefab);
        //m_ClusterUIScript.Update();
    }

    TreeScript GetTree()
    {
        return treePrefab.GetComponent<TreeScript>();
    }
}
