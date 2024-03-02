using Clam;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

public class GraphBuilder : MonoBehaviour
{
    private Vector3[] m_Vertices;
    private int[] m_Indices;
    private int m_VertexCounter;
    private int m_IndexCounter;
    private bool m_IsPhysicsRunning;
    Dictionary<string, GameObject> m_Graph;

    //private float m_EdgeScalar = 25.0f;

    // Start is called before the first frame update
    void Start()
    {
        m_VertexCounter = 0;
        m_IndexCounter = 0;
        Debug.Log("physics is running start val " + m_IsPhysicsRunning);
        MenuEventManager.StartListening(Menu.DestroyGraph, DestroyGraph);
    }

    public void DestroyGraph()
    {
        GetComponent<MeshFilter>().mesh = new Mesh();
    }

    // Update is called once per frame
    void Update()
    {
        if (m_IsPhysicsRunning)
        {
            if (Clam.FFI.NativeMethods.PhysicsUpdateAsync(PositionUpdater) == FFIError.PhysicsFinished)
            {
                m_IsPhysicsRunning = false;
                MenuEventManager.instance.m_IsPhysicsRunning = false;
                //Clam.FFI.NativeMethods.RunTriangleTest(false, m_OutTestFilePath, clusterGetter);
            }
        }
    }

    public void Init(System.Collections.Generic.Dictionary<string, GameObject> graph, float edgeScalar, int numIters)
    {
        m_Graph = graph;
        GetComponent<MeshFilter>().mesh = new Mesh();
        var buildResult = Clam.FFI.NativeMethods.InitForceDirectedGraph(edgeScalar, numIters);
        if (buildResult != FFIError.Ok)
        {
            UIHelpers.ShowErrorPopUP("Graph build failed");
            Debug.LogError("Graph build failed");
            return;
        }
        m_VertexCounter = 0;
        m_IndexCounter = 0;

        int numNodes = m_Graph.Count;
        int numEdges = Clam.FFI.NativeMethods.GetNumGraphEdges();
        Debug.Log("num edges in graph : " + numEdges + ", num nodes " + numNodes);

        m_Vertices = new Vector3[numNodes];
        m_Indices = new int[numEdges * 2];
        InitNodeIndices();
        Clam.FFI.NativeMethods.InitGraphVertices(EdgeDrawer);

        m_IsPhysicsRunning = true;
        // terrible redesign later*********************************************************
        MenuEventManager.instance.m_IsPhysicsRunning = true;

        Debug.Log("physics is runninginit val " + m_IsPhysicsRunning);
    }

    void InitNodeIndices()
    {
        int i = 0;
        foreach (var (id, node) in m_Graph)
        {
            node.GetComponent<Node>().IndexBufferID = i;
            m_Vertices[i] = node.GetComponent<Node>().GetPosition();
            i++;
        }

        var mesh = GetComponent<MeshFilter>().mesh;
        mesh.vertices = m_Vertices;
        mesh.RecalculateBounds();
    }

    public void PositionUpdater(ref Clam.FFI.ClusterData nodeData)
    {
        string id = nodeData.id.AsString;
        if (m_Graph.TryGetValue(id, out var node))
        {
            node.GetComponent<Node>().SetPosition(nodeData.pos.AsVector3);

            m_Vertices[node.GetComponent<Node>().IndexBufferID] = node.GetComponent<Node>().GetPosition();
            m_VertexCounter++;

            if (m_VertexCounter == m_Vertices.Length)
            {
                var mesh = GetComponent<MeshFilter>().mesh;
                mesh.vertices = m_Vertices;
                mesh.RecalculateBounds();
                m_VertexCounter = 0;
            }
        }
        else
        {
            Debug.Log("physics upodate key not found - " + id);
        }
    }

    public void EdgeDrawer(ref Clam.FFI.ClusterData nodeData)
    {
        string msg = nodeData.message.AsString;
        var values = msg.Split(' ').ToList();
        string otherID = values[1];
        bool isDetected = values[0][0] == '1';

        if (m_Graph.TryGetValue(nodeData.id.AsString, out var node))
        {
            if (m_Graph.TryGetValue(otherID, out var other))
            {
                if (isDetected)
                {
                    var id1 = node.GetComponent<Node>().IndexBufferID;
                    var id2 = other.GetComponent<Node>().IndexBufferID;
                    m_Indices[m_IndexCounter++] = id1;
                    m_Indices[m_IndexCounter++] = id2;
                    if (m_IndexCounter == m_Indices.Length)
                    {
                        GetComponent<MeshFilter>().mesh.SetIndices(m_Indices, MeshTopology.Lines, 0);
                        Debug.Log("all edges drawn");

                    }
                }
            }
        }
    }

    public void ToggleEdgeVisibility(bool value)
    {
        Debug.Log("toggling edge visibility");
        GetComponent<Renderer>().enabled = value;
    }
}
