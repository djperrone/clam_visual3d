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
    private float m_EdgeScalar = 25.0f;


    //int m_ShouldUpdate = 0;
    //private bool m_InitializedIndices;
    //private bool m_UpdatedAllVertices;

    // Start is called before the first frame update
    void Start()
    {
        m_VertexCounter = 0;
        m_IndexCounter = 0;
        //m_IsPhysicsRunning = false;
        //lineMesh = new Mesh();
        //GetComponent<MeshFilter>().mesh = lineMesh;

        Debug.Log("physics is running start val " + m_IsPhysicsRunning);
        MenuEventManager.StartListening(Menu.DestroyGraph, DestroyGraph);
    }
    void DestroyGraph()
    {
        GetComponent<MeshFilter>().mesh = new Mesh();
        //m_Vertices = null;
        //m_Vertices = new Vector3[0];
        //m_Indices = new int[0];

        //mesh.vertices = m_Vertices;
        ////mesh.triangles = m_Indices;
        //GetComponent<MeshFilter>().mesh.SetIndices(m_Indices, MeshTopology.Lines, 0);
        //mesh.RecalculateBounds();

    }
    // Update is called once per frame
    void Update()
    {
        //Debug.Log("graph builder update");
        //Debug.Log("physics is running updatet val " + m_IsPhysicsRunning);

        //m_VertexCounter = 0;
        if (m_IsPhysicsRunning)
        {
            //Debug.Log("graph builder updatephysics is runningf");

            if (Clam.FFI.NativeMethods.PhysicsUpdateAsync(PositionUpdater) == FFIError.PhysicsFinished)
            {
                m_IsPhysicsRunning = false;
                MenuEventManager.instance.m_IsPhysicsRunning = false;
                print("physics finished");
            }
        }

    }

    public void Init(Clam.FFI.ClusterData[] nodes, float edgeScalar, int numIters)
    {
        GetComponent<MeshFilter>().mesh = new Mesh();

        Clam.FFI.NativeMethods.InitForceDirectedGraph(nodes, edgeScalar, numIters);

        m_VertexCounter = 0;
        m_IndexCounter = 0;

        //GetComponent<MeshFilter>().mesh = new Mesh();

        int numNodes = Cakes.Tree.GetTree().Count;
        int numEdges = Clam.FFI.NativeMethods.GetNumEdgesInGraph();
        Debug.Log("num edges in graph : " + numEdges + ", num nodes " + numNodes);
        m_Vertices = new Vector3[numNodes];
        m_Indices = new int[numEdges * 2];
        InitNodeIndices();
        Clam.FFI.NativeMethods.InitGraphVertices(EdgeDrawer);



        for (int K = 0; K < nodes.Length; K++)
        {
            //ref var node = ref node1;
            //Debug.Log("freeing all nodes from physics sim");
            Clam.FFI.NativeMethods.DeleteClusterData(ref nodes[K]);
        }
        //Clam.FFI.NativeMethods.RunForceDirectedSim(nodes, edgeScalar, numIters, EdgeDrawer);

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

        //Debug.Log("graph builder update");
        string id = nodeData.id.AsString;
        //Debug.Log("id of updated node is " + id);
        if (Cakes.Tree.GetTree().TryGetValue(id, out var node))
        {
            node.GetComponent<Node>().SetPosition(nodeData.pos.AsVector3);

            int numNodes = Cakes.Tree.GetTree().Count;


            m_Vertices[node.GetComponent<Node>().IndexBufferID] = node.GetComponent<Node>().GetPosition();
            m_VertexCounter++;

            if (m_VertexCounter == m_Vertices.Length)
            {
                //m_UpdatedAllVertices = true;
                var mesh = GetComponent<MeshFilter>().mesh;
                mesh.vertices = m_Vertices;
                mesh.RecalculateBounds();
                m_VertexCounter = 0;

                Debug.Log("set all line vertices in update");
            }
        }
        else
        {
            Debug.Log("physics upodate key not found - " + id);
        }
    }

    public void EdgeDrawer(ref Clam.FFI.ClusterData nodeData)
    {

        //Debug.Log("graph builder edge drawer");
        string msg = nodeData.message.AsString;
        var values = msg.Split(' ').ToList();
        string otherID = values[1];
        bool isDetected = values[0][0] == '1';
        //Debug.Log("val 0 " + vals[0]);
        //Debug.Log("val 1 " + vals[1]);

        if (Cakes.Tree.GetTree().TryGetValue(nodeData.id.AsString, out var node))
        {
            if (Cakes.Tree.GetTree().TryGetValue(otherID, out var other))
            {
                Debug.Log("other id valis");
                if (isDetected)
                {
                    Debug.Log("is detected is true");

                    var id1 = node.GetComponent<Node>().IndexBufferID;
                    var id2 = other.GetComponent<Node>().IndexBufferID;
                    m_Indices[m_IndexCounter++] = id1;
                    m_Indices[m_IndexCounter++] = id2;
                    //m_Vertices[id1] = node.ge;
                    //m_Vertices[id2] = nodeData.pos.AsVector3;
                    if (m_IndexCounter == m_Indices.Length)
                    {
                        //m_InitializedIndices = true;
                        GetComponent<MeshFilter>().mesh.SetIndices(m_Indices, MeshTopology.Lines, 0);
                        //m_Indices = null;
                        Debug.Log("set indices in edge drawer");
                    }
                }
            }
        }
        Debug.Log("graph builder edge drawer should never hit this");

    }

    public void ToggleEdgeVisibility(bool value)
    {
        Debug.Log("toggling edge visinility");
        GetComponent<Renderer>().enabled = value;
    }
}
