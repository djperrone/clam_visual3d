using Clam;
using Clam.FFI;
using System.Linq;
using UnityEngine;

public class TreeLayout
{
    string m_RootID;
    int m_RootDepth;
    int m_CurrentDepth;
    int m_MaxDepth;
    int m_IntervalStep = 3;    // depth given to reingold will be max-rootDepth

    public int CurrentDepth()
    {
        return m_CurrentDepth;
    }

    public TreeLayout(string rootId)
    {
        m_RootID = rootId;

        var dataWrapper = new RustResourceWrapper<ClusterData>(ClusterData.Alloc(rootId));
        m_RootDepth = dataWrapper.Data.depth;

        Debug.Log("root depth: " + m_RootDepth);

        m_CurrentDepth = m_RootDepth + 1;
        m_MaxDepth = m_CurrentDepth + m_IntervalStep;

        NativeMethods.DrawHierarchyOffsetFrom(new RustResourceWrapper<ClusterData>(ClusterData.Alloc(m_RootID)), UpdatePositionCallback, m_RootDepth, m_CurrentDepth, m_MaxDepth);

        NativeMethods.ForEachDFT(ClusterVisibilityCallback, m_RootID);
        NativeMethods.ForEachDFT(EdgeVisibilityCallback);

        MenuEventManager.StartListening(Menu.ShowEntireTree, ShowAll);
    }

    public void ShowMore()
    {
        int nextDepth = m_CurrentDepth + 1;
        if (nextDepth > NativeMethods.TreeHeight())
        {
            Debug.Log("Tree layout height is at max");
            return;
        }
        m_CurrentDepth++;

        if (m_CurrentDepth > m_MaxDepth)
        {
            Debug.Log("Increasing size");
            m_MaxDepth += m_IntervalStep;
            NativeMethods.DrawHierarchyOffsetFrom(new RustResourceWrapper<ClusterData>(ClusterData.Alloc(m_RootID)), UpdatePositionCallback, m_RootDepth, m_CurrentDepth, m_MaxDepth);
        }

        UpdateNodeVisibility(nextDepth);
    }

    public void ShowAll()
    {
        Debug.Log("Showing whole tree");
        m_MaxDepth = Clam.FFI.NativeMethods.TreeHeight();
        m_CurrentDepth = m_MaxDepth;
        NativeMethods.DrawHierarchyOffsetFrom(new RustResourceWrapper<ClusterData>(ClusterData.Alloc(m_RootID)), UpdatePositionCallback, m_RootDepth, m_CurrentDepth, m_MaxDepth);
        NativeMethods.ForEachDFT(ClusterVisibilityCallback, m_RootID, m_MaxDepth);
        NativeMethods.ForEachDFT(EdgeVisibilityCallback, m_RootID, m_MaxDepth);

    }
    public void ShowLess()
    {
        int nextDepth = m_CurrentDepth - 1;

        if (nextDepth <= 0)
        {
            Debug.Log("Tree height is at min");
            return;
        }
        m_CurrentDepth--;

        if (m_CurrentDepth < m_MaxDepth - m_IntervalStep)
        {
            Debug.Log("Decreasing size");

            m_MaxDepth -= m_IntervalStep;
            NativeMethods.DrawHierarchyOffsetFrom(new RustResourceWrapper<ClusterData>(ClusterData.Alloc(m_RootID)), UpdatePositionCallback, m_RootDepth, m_CurrentDepth, m_MaxDepth);
        }

        UpdateNodeVisibility(nextDepth);
    }

    void UpdateNodeVisibility(int newDepth)
    {
        NativeMethods.ForEachDFT(ClusterVisibilityCallback, m_RootID, m_CurrentDepth + 1);
        NativeMethods.ForEachDFT(EdgeVisibilityCallback, m_RootID, m_CurrentDepth + 1);
    }

    void UpdatePositionCallback(ref ClusterData data)
    {
        var id = data.id.AsString;
        GameObject clusterObject = Cakes.Tree.GetOrAdd(id);
        clusterObject.GetComponent<Clam.Node>().SetPosition(data.pos.AsVector3);
    }

    void ClusterVisibilityCallback(ref ClusterData data)
    {
        var id = data.id.AsString;
        GameObject clusterObject = Cakes.Tree.GetOrAdd(id);
        if (data.depth <= m_CurrentDepth)
        {
            clusterObject.SetActive(true);
        }
        else
        {
            clusterObject.SetActive(false);
        }
    }

    public void EdgeVisibilityCallback(ref Clam.FFI.ClusterData nodeData)
    {
        if (Cakes.Tree.GetTree().TryGetValue(nodeData.id.AsString, out var node))
        {
            if (node.activeSelf && !node.GetComponent<Node>().IsLeaf())
            {
                UpdateEdgeVisibility(node.GetComponent<Node>().GetLeftChildID(), node);
                UpdateEdgeVisibility(node.GetComponent<Node>().GetRightChildID(), node);
            }
        }
    }

    private void UpdateEdgeVisibility(string childId, GameObject parentNode)
    {
        if (Cakes.Tree.GetTree().TryGetValue(childId, out var childNode) && childNode.activeSelf)
        {
            string edgeKey = parentNode.GetComponent<Node>().GetId() + childNode.GetComponent<Node>().GetId();

            if (Cakes.Tree.GetEdges().TryGetValue(edgeKey, out var edgeGameObject) && edgeGameObject != null && !edgeGameObject.activeSelf)
            {
                edgeGameObject.SetActive(true);
            }
        }
    }

    // Start is called before the first frame update
    void Start()
    {

    }

    // Update is called once per frame
    void Update()
    {

    }
}
