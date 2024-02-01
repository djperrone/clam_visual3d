using Clam;
using System.Linq;
using UnityEngine;

public class GraphBuilder : MonoBehaviour
{
    private Vector3[] m_Vertices;
    private int[] m_Indices;
    private int m_VertexCounter;
    private int m_IndexCounter;
    private bool m_IsPhysicsRunning;
    //private float m_EdgeScalar = 25.0f;

    // Start is called before the first frame update
    void Start()
    {
        m_VertexCounter = 0;
        m_IndexCounter = 0;
        Debug.Log("physics is running start val " + m_IsPhysicsRunning);
        MenuEventManager.StartListening(Menu.DestroyGraph, DestroyGraph);
    }
    void DestroyGraph()
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
            }
        }
    }

    public void Init(Clam.FFI.ClusterData[] nodes, float edgeScalar, int numIters)
    {
        GetComponent<MeshFilter>().mesh = new Mesh();
        Clam.FFI.NativeMethods.InitForceDirectedGraph(nodes, edgeScalar, numIters);

        m_VertexCounter = 0;
        m_IndexCounter = 0;

        int numNodes = Cakes.Tree.GetTree().Count;
        int numEdges = Clam.FFI.NativeMethods.GetNumGraphEdges();
        Debug.Log("num edges in graph : " + numEdges + ", num nodes " + numNodes);

        if (numEdges <2) {
            UIHelpers.ShowErrorPopUP("less than 2 edges in graph");
            Debug.LogWarning("less than 2 edges in graph");
            for (int K = 0; K < nodes.Length; K++)
            {
                Clam.FFI.NativeMethods.DeleteClusterData(ref nodes[K]);
            }
            return;
        }
        m_Vertices = new Vector3[numNodes];
        m_Indices = new int[numEdges * 2];
        InitNodeIndices();
        Clam.FFI.NativeMethods.InitGraphVertices(EdgeDrawer);

        for (int K = 0; K < nodes.Length; K++)
        {
            Clam.FFI.NativeMethods.DeleteClusterData(ref nodes[K]);
        }

        m_IsPhysicsRunning = true;
        // terrible redesign later*********************************************************
        MenuEventManager.instance.m_IsPhysicsRunning = true;

        Debug.Log("physics is runninginit val " + m_IsPhysicsRunning);
    }

    void InitNodeIndices()
    {
        int i = 0;
        foreach (var (id, node) in Cakes.Tree.GetTree())
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
        if (Cakes.Tree.GetTree().TryGetValue(id, out var node))
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

        if (Cakes.Tree.GetTree().TryGetValue(nodeData.id.AsString, out var node))
        {
            if (Cakes.Tree.GetTree().TryGetValue(otherID, out var other))
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
