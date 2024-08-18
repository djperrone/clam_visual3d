using Clam;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;



public class Edge : MonoBehaviour
{
    public enum SpringType
    {
        None = 0,
        heirarchal,
        Similarity,
        
    }
    public GameObject m_NodePrefab;
    private GameObject m_Node1;
    private GameObject m_Node2;

    private LineRenderer m_LineRenderer;

    //public static SpringScript CreateInstance(GameObject node1, GameObject node2, SpringType springType)
    //{
    //    GameObject instance = Instantiate()
    //    instance.InitLineRenderer();
    //    instance.m_Node1 = node1;
    //    instance.m_Node2 = node2;

    //    if (springType == SpringType.heirarchal)
    //    {
    //        instance.SetColor(Color.white);
    //    }
    //    else
    //    {
    //        instance.SetColor(Color.black);
    //    }

    //    return instance;

    //}
    public void InitLineRenderer(GameObject node1, GameObject node2, SpringType springType)
    {
        //m_LineRenderer = GetComponent<LineRenderer>();
        //m_LineRenderer.startWidth = 0.1f;
        //m_LineRenderer.endWidth = 0.1f;
        //m_LineRenderer.useWorldSpace = true;
        gameObject.name = (node1.GetComponent<Node>().GetId(),node2.GetComponent<Node>().GetId()).ToString();

        m_Node1 = node1;
        m_Node2 = node2;

        if (springType == SpringType.heirarchal)
        {
            SetColor(Color.white);
        }
        else
        {
            SetColor(Color.black);
        }

        List<Vector3> positions = new List<Vector3>() { m_Node1.transform.position, m_Node2.transform.position };
        GetComponent<LineRenderer>().SetPositions(positions.ToArray());
    }
    // Start is called before the first frame update
    void Start()
    {
        m_LineRenderer = GetComponent<LineRenderer>();
        m_LineRenderer.startWidth = 0.1f;
        m_LineRenderer.endWidth = 0.1f;
        m_LineRenderer.useWorldSpace = true;
        //l.startColor = Color.black;
        //l.endColor = Color.black;
        //m_LineRenderer.material.color = Color.red;
        //m_LineRenderer.startColor = Color.black;
        //m_LineRenderer.endColor = Color.black;
    }

    // Update is called once per frame
    void Update()
    {
        if (!m_Node1.activeSelf || !m_Node2.activeSelf)
        {
            this.gameObject.SetActive(false);
            return;
        }
        List<Vector3> positions = new List<Vector3>() { m_Node1.transform.position, m_Node2.transform.position };
        GetComponent<LineRenderer>().SetPositions(positions.ToArray());
    }

    public void SetNodes(GameObject node1, GameObject node2)
    {
        m_Node1 = node1;
        m_Node2 = node2;
    }

    public (GameObject, GameObject) GetNodes()
    {
        return (m_Node1, m_Node2);
    }

    public void SetColor(Color color)
    {
        GetComponent<LineRenderer>().startColor = color;
        GetComponent<LineRenderer>().endColor = color;
        //m_LineRenderer = GetComponent<LineRenderer>();
        //var renderer = m_LineRenderer;
        //var m = renderer.GetComponent<Material>;
        //var c = m.color;
        //m_LineRenderer.material.color = color;

    }
}
