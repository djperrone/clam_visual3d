using Clam;
using Clam.FFI;
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
    Dictionary<(nuint, nuint), GameObject> m_Graph;

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
            if (Clam.FFI.NativeMethods.PhysicsUpdateAsync(ResetGraphCallback, RefillGraphCallback, ResetGraphMesh, EdgeDrawer, PositionUpdater) == FFIError.PhysicsFinished)
            {
                m_IsPhysicsRunning = false;
                MenuEventManager.instance.m_IsPhysicsRunning = false;
                //Clam.FFI.NativeMethods.RunTriangleTest(false, m_OutTestFilePath, clusterGetter);
            }
        }
    }

    public void Init(System.Collections.Generic.Dictionary<(nuint, nuint), GameObject> graph, float edgeScalar, int numIters)
    {
        m_Graph = new Dictionary<(nuint, nuint), GameObject>();
        (FFIError result, ClusterData root) = NativeMethods.GetRootData();
        if (result != FFIError.Ok)
        {
            UIHelpers.ShowErrorPopUP("Root not found");
            Debug.LogError("Root not found");
            return;
        }

        var rootObject = Cakes.Tree.GetOrAdd(root.ID().AsTuple());
        var leftID = rootObject.GetComponent<Node>().GetLeftChildID();
        var rightID = rootObject.GetComponent<Node>().GetRightChildID();

        m_Graph[leftID] = Cakes.Tree.GetOrAdd(leftID);
        m_Graph[rightID] = Cakes.Tree.GetOrAdd(rightID);

        foreach ((var id, var node) in Cakes.Tree.GetTree())
        {
            if (!m_Graph.ContainsKey(id))
            {
                //GameObject.Destroy(node);
                node.SetActive(false);
            }
        }

        //GetComponent<MeshFilter>().mesh = new Mesh();
        var buildResult = Clam.FFI.NativeMethods.InitForceDirectedGraph(edgeScalar, numIters);
        if (buildResult != FFIError.Ok)
        {
            UIHelpers.ShowErrorPopUP("Graph build failed");
            Debug.LogError("Graph build failed");
            return;
        }
        m_VertexCounter = 0;
        m_IndexCounter = 0;

        int numNodes = 2;
        int numEdges = 1;
        //int numEdges = Clam.FFI.NativeMethods.GetNumGraphEdges();

        if (numEdges < 0)
        {
            Debug.LogError("num edges error"); return;
        }
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

    public void ResetGraphCallback(ref Clam.FFI.ClusterData nodeData)
    {
        foreach ((var id, var node) in m_Graph)
        {
            if (m_Graph.ContainsKey(id))
            {
                node.SetActive(false);
                node.GetComponent<Node>().Deselect();
            }
        }
        m_Graph = new Dictionary<(nuint, nuint), GameObject>();
    }

    public void RefillGraphCallback(ref Clam.FFI.ClusterData nodeData)
    {
        var id = nodeData.ID_AsTuple();
        var cluster = Cakes.Tree.GetOrAdd(id);
        cluster.SetActive(true);
        m_Graph[id] = cluster;
        cluster.GetComponent<Node>().SetPosition(nodeData.pos.AsVector3);
        cluster.GetComponent<Node>().Deselect();    
    }

    // MAKE INTO CALLBACK TO RUST
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

    void ResetGraphMesh(ref Clam.FFI.ClusterData nodeData)
    {
        //m_VertexCounter = 0;
        m_IndexCounter = 0;
        int numNodes = m_Graph.Count;
        int numEdges = nodeData.depth;
        if (numEdges < 0)
        {
            Debug.LogError("num edges error"); return;
        }
        Debug.Log("num edges in graph : " + numEdges + ", num nodes " + numNodes);

        m_Vertices = new Vector3[numNodes];
        m_Indices = new int[numEdges * 2];

        int i = 0;
        InitNodeIndices();
        Debug.Log("About to update edges");


    }

    //public void RebuildEdges(ref Clam.FFI.ClusterIDs nodeData)
    //{
    //    Clam.FFI.NativeMethods.InitGraphVertices(EdgeDrawer);

    //}

    public void PositionUpdater(ref Clam.FFI.ClusterData nodeData)
    {
        var id = nodeData.ID_AsTuple();
        //if (Cakes.Tree.GetTree().TryGetValue(id, out var node))
        if (m_Graph.TryGetValue(id, out var node))
        {
            node.SetActive(true);
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

    public void EdgeDrawer(ref Clam.FFI.ClusterIDs nodeData)
    {
        Debug.Log("Updating edges for graph " + m_Graph.Count.ToString());

        // If the edge is a chaoda detected edge
        if (nodeData.rightID.Offset == 1)
        {
            if (m_IndexCounter < m_Indices.Length)
            {
                if (Cakes.Tree.GetTree().TryGetValue(nodeData.id.AsTuple(), out var node))
                {
                    if (Cakes.Tree.GetTree().TryGetValue(nodeData.leftID.AsTuple(), out var other))
                    {
                        var id1 = node.GetComponent<Node>().IndexBufferID;
                        var id2 = other.GetComponent<Node>().IndexBufferID;
                        m_Indices[m_IndexCounter++] = id1;
                        m_Indices[m_IndexCounter++] = id2;
                        if (m_IndexCounter == m_Indices.Length)
                        {
                            GetComponent<MeshFilter>().mesh.SetIndices(m_Indices, MeshTopology.Lines, 0);
                            Debug.Log("all edges drawn" + m_Graph.Count.ToString());
                            m_IndexCounter = 0;
                        }

                    }
                }
            }
            else
            {
                Debug.Log("tHIS SHOULDNT RUN!!!! index counter" + m_IndexCounter.ToString() + ", / " + Clam.FFI.NativeMethods.GetNumGraphEdges().ToString());
                m_IndexCounter++;
                m_IndexCounter++;
            }
        }

    }

    public void ToggleEdgeVisibility(bool value)
    {
        Debug.Log("toggling edge visibility");
        GetComponent<Renderer>().enabled = value;
    }
}
