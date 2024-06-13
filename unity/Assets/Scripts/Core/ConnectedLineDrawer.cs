
using UnityEngine;

public class ConnectedLineDrawer : MonoBehaviour
{
    public int lineCount = 3000; // Adjust the number of lines as needed

    private Mesh lineMesh;
    private Vector3[] lineVertices;

    void Start()
    {
        InitializeLineMesh();
        UpdateLineMesh();
    }

    void InitializeLineMesh()
    {
        lineMesh = new Mesh();
        GetComponent<MeshFilter>().mesh = lineMesh;

        // Initialize line vertices (assuming you have a simple line mesh)
        lineVertices = new Vector3[lineCount * 2];
        for (int i = 0; i < lineCount; i++)
        {
            lineVertices[i * 2] = Random.onUnitSphere * 5f; // Example: Random positions
            lineVertices[i * 2 + 1] = Random.onUnitSphere * 5f; // Example: Random positions
        }

        lineMesh.vertices = lineVertices;

        // Assuming you have a simple line mesh (two vertices per line)
        int[] lineIndices = new int[lineCount * 2];
        for (int i = 0; i < lineCount * 2; i++)
        {
            lineIndices[i] = i;
        }

        lineMesh.SetIndices(lineIndices, MeshTopology.Lines, 0);
    }

    void UpdateLineMesh()
    {
        // You can use the Standard Shader for simplicity
        Renderer rend = GetComponent<Renderer>();
        rend.material = new Material(Shader.Find("Standard"));

        // Assign the material to the mesh
        rend.sharedMaterial = rend.material;
    }

    void Update()
    {
        // Simulate dynamic movement by updating line positions each frame
        for (int i = 0; i < lineCount; i++)
        {
            Vector3 startPoint = Random.onUnitSphere * 5f;
            Vector3 endPoint = startPoint + Random.onUnitSphere * 5f;

            lineVertices[i * 2] = startPoint;
            lineVertices[i * 2 + 1] = endPoint;
        }

        lineMesh.vertices = lineVertices;
    }
}

//using System.Collections;
//using System.Collections.Generic;
//using UnityEngine;

//public class ConnectedLineDrawer : MonoBehaviour
//{
//    private LineRenderer lineRenderer;

//    public float lineThickness = 50f;

//    void Start()
//    {
//        // Create a new mesh
//        Mesh mesh = new Mesh();

//        // Define the vertices for a rectangle (thickness is controlled by the Y component)
//        float halfThickness = lineThickness * 0.5f;
//        Vector3[] vertices = new Vector3[]
//        {
//            new Vector3(0, -halfThickness, 0),
//            new Vector3(1, -halfThickness, 1),
//            new Vector3(0, halfThickness, 0),
//            new Vector3(1, halfThickness, 1)
//        };

//        // Extrude the vertices along the normals to create thickness
//        Vector3[] extrudedVertices = new Vector3[vertices.Length * 2];
//        for (int i = 0; i < vertices.Length; i++)
//        {
//            extrudedVertices[i] = vertices[i];
//            extrudedVertices[i + vertices.Length] = vertices[i] + Vector3.up * lineThickness;
//        }

//        // Define the indices to form line segments
//        int[] indices = new int[] { 0, 1, 2, 3, 0, 2, 1, 3 };

//        // Set the vertices and indices to the mesh
//        mesh.vertices = extrudedVertices;
//        mesh.SetIndices(indices, MeshTopology.Lines, 0);

//        // Recalculate bounds (optional, but can be useful for culling)
//        mesh.RecalculateBounds();

//        // Create a new game object with MeshFilter and MeshRenderer components
//        GameObject lineObject = new GameObject("LineMesh");
//        MeshFilter meshFilter = lineObject.AddComponent<MeshFilter>();
//        MeshRenderer meshRenderer = lineObject.AddComponent<MeshRenderer>();

//        // Assign the mesh to the MeshFilter
//        meshFilter.mesh = mesh;

//        // Assign a material to the MeshRenderer
//        meshRenderer.material = new Material(Shader.Find("Standard"));

//        // You can set additional material properties or adjust the object's transform as needed
//    }

//    //public float lineThickness = 50f;

//    //void Start()
//    //{
//    //    // Create a new mesh
//    //    Mesh mesh = new Mesh();

//    //    // Define the vertices for a rectangle (thickness is controlled by the Y component)
//    //    float halfThickness = lineThickness * 0.5f;
//    //    Vector3[] vertices = new Vector3[]
//    //    {
//    //        new Vector3(0, -halfThickness, 0),
//    //        new Vector3(1, -halfThickness, 1),
//    //        new Vector3(0, halfThickness, 0),
//    //        new Vector3(1, halfThickness, 1)
//    //    };

//    //    // Define the indices to form line segments
//    //    int[] indices = new int[] { 0, 1, 2, 3, 0, 2, 1, 3 };

//    //    // Set the vertices and indices to the mesh
//    //    mesh.vertices = vertices;
//    //    mesh.SetIndices(indices, MeshTopology.Lines, 0);

//    //    // Recalculate bounds (optional, but can be useful for culling)
//    //    mesh.RecalculateBounds();

//    //    // Create a new game object with MeshFilter and MeshRenderer components
//    //    GameObject lineObject = new GameObject("LineMesh");
//    //    MeshFilter meshFilter = lineObject.AddComponent<MeshFilter>();
//    //    MeshRenderer meshRenderer = lineObject.AddComponent<MeshRenderer>();

//    //    // Assign the mesh to the MeshFilter
//    //    meshFilter.mesh = mesh;

//    //    // Assign a material to the MeshRenderer
//    //    meshRenderer.material = new Material(Shader.Find("Standard"));

//    //    // You can set additional material properties or adjust the object's transform as needed
//    //}
//    //void Start()
//    //{
//    //    // Create a new mesh
//    //    Mesh mesh = new Mesh();

//    //    // Define the vertices for the two parallel lines
//    //    Vector3[] vertices = new Vector3[]
//    //    {
//    //        new Vector3(0, 0, 0),
//    //        new Vector3(1, 1, 1),
//    //        new Vector3(0, 1, 0),
//    //        new Vector3(1, 2, 1)
//    //    };

//    //    // Define the indices to form line segments
//    //    int[] indices = new int[] { 0, 1, 2, 3 };

//    //    // Set the vertices and indices to the mesh
//    //    mesh.vertices = vertices;
//    //    mesh.SetIndices(indices, MeshTopology.Lines, 0);

//    //    // Recalculate bounds (optional, but can be useful for culling)
//    //    mesh.RecalculateBounds();

//    //    // Create a new game object with MeshFilter and MeshRenderer components
//    //    GameObject lineObject = new GameObject("ParallelLinesMesh");
//    //    MeshFilter meshFilter = lineObject.AddComponent<MeshFilter>();
//    //    MeshRenderer meshRenderer = lineObject.AddComponent<MeshRenderer>();

//    //    // Assign the mesh to the MeshFilter
//    //    meshFilter.mesh = mesh;

//    //    // Assign a material to the MeshRenderer
//    //    meshRenderer.material = new Material(Shader.Find("Standard"));

//    //    // You can set additional material properties or adjust the object's transform as needed
//    //}
//    //private LineRenderer lineRenderer;

//    //void Start()
//    //{
//    //    lineRenderer = gameObject.AddComponent<LineRenderer>();
//    //    lineRenderer.positionCount = 4;

//    //    // Set material and color
//    //    lineRenderer.material = new Material(Shader.Find("Standard"));
//    //    lineRenderer.material.color = Color.red;

//    //    // Set line positions
//    //    Vector3[] linePositions = new Vector3[]
//    //    {
//    //        new Vector3(0, 0, 0),
//    //        new Vector3(1, 1, 1),
//    //        new Vector3(2, 0, 0),
//    //        new Vector3(3, 1, 1)
//    //    };

//    //    SetLinePositions(linePositions);
//    //}

//    //void SetLinePositions(Vector3[] positions)
//    //{
//    //    lineRenderer.SetPositions(positions);
//    //}
//}
