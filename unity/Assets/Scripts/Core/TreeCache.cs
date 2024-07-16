using Clam;
using Clam.FFI;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;


namespace Clam
{
    public class TreeCache : MonoBehaviour
    {
        public GameObject m_NodePrefab;
        public GameObject m_SpringPrefab;

        private string m_DataName;
        private uint m_Cardinality;

        public TreeStartupData m_TreeData;

        private Dictionary<(nuint, nuint), GameObject> m_Tree;
        private Dictionary<((nuint, nuint), (nuint, nuint)), GameObject> m_EdgeCache;
        public bool m_IsPhysicsRunning = false;

        public FFIError Init(GameObject nodePrefab, GameObject springPrefab)
        {
            m_NodePrefab = nodePrefab;
            m_SpringPrefab = springPrefab;
            m_TreeData = MenuEventManager.instance.m_TreeData;
            if (m_TreeData.dataName == null || m_TreeData.dataName.Length == 0)
            {
                Debug.Log("error with tree data");
                return FFIError.StartupDataInvalid;
            }

            if (m_TreeData.shouldLoad)
            {
                FFIError clam_result = Clam.FFI.NativeMethods.LoadCakes(m_TreeData);
                if (clam_result != FFIError.Ok)
                {
                    Debug.Log("error with tree data");
                    return clam_result;
                }
            }
            else
            {
                FFIError clam_result = Clam.FFI.NativeMethods.InitClam(m_TreeData);
                if (clam_result != FFIError.Ok)
                {
                    Debug.Log("error with tree data");
                    return clam_result;
                }
            }

            m_Tree = new Dictionary<(nuint, nuint), GameObject>();
            m_EdgeCache = new Dictionary<((nuint, nuint), (nuint, nuint)), GameObject>();

            FFIError e = Clam.FFI.NativeMethods.SetNames(SetNodeNames, 0, NativeMethods.TreeCardinality());

            if (e == FFIError.Ok)
            {
                Debug.Log("set names complete)");
            }
            else
            {
                Debug.Log("ERROR " + e);
            }
            Clam.FFI.NativeMethods.DrawHierarchy(PositionUpdater);
            //Clam.FFI.NativeMethods.ColorClustersByEntropy(ColorFiller);
            var colorErr = Clam.FFI.NativeMethods.ColorClustersByDominantLabel(ColorFiller);
            if (colorErr != FFIError.Ok)
            {
                return FFIError.ColoringFailed;
            }


            Clam.FFI.NativeMethods.ForEachDFT(EdgeDrawer, new ClusterID(0, NativeMethods.TreeCardinality()));

            (FFIError err, ClusterData rootData) = NativeMethods.GetRootData();

            if (err == FFIError.Ok)
            {
                Debug.Log(System.String.Format("created tree with num nodes {0}.", rootData.cardinality));
            }
            else
            {
                Debug.LogError("root not found?");
                return FFIError.HandleInitFailed;
            }

            return FFIError.Ok;
        }

  

        public void ResetTree()
        {
            FFIError e = Clam.FFI.NativeMethods.SetNames(SetNodeNames, 0, NativeMethods.TreeCardinality());

            if (e == FFIError.Ok)
            {
                Debug.Log("ok)");
            }
            else
            {
                Debug.Log("ERROR " + e);
            }


            DestroyEdges();

            Clam.FFI.NativeMethods.DrawHierarchy(PositionUpdater);
            Clam.FFI.NativeMethods.ColorClustersByDominantLabel(ColorFiller);
            Clam.FFI.NativeMethods.ForEachDFT(EdgeDrawer, new ClusterID(0, NativeMethods.TreeCardinality()));
            //PopulateEdgeDictionary();
        }

        public void DestroyEdges()
        {
            //var edges = GameObject.FindObjectsOfType<Edge>(true);
            //Debug.Log("before num edges" + edges.Length.ToString());
            //foreach(var edge in edges)
            //{
            //    GameObject.Destroy(edge.gameObject);
            //}
            //edges = GameObject.FindObjectsByType<Edge>(FindObjectsInactive.Include, FindObjectsSortMode.None);
            //Debug.Log("after1 num edges" + edges.Length.ToString());
            //Debug.Log("edge cache cnt: " + m_EdgeCache.Count.ToString());   
            foreach ((var id, GameObject edge) in m_EdgeCache)
            {
                Destroy(edge);
            }

            //edges = GameObject.FindObjectsOfType<Edge>(true);
            //Debug.Log("after2 num edges" + edges.Length.ToString());
            //m_EdgeCache.Clear();
            m_EdgeCache = new Dictionary<((nuint, nuint), (nuint, nuint)), GameObject>();
        }

        public void EdgeDrawer(ref FFI.ClusterData nodeData)
        {
            Debug.Log("Edge Drawer");
            if (m_Tree.TryGetValue(nodeData.ID_AsTuple(), out var node))
            {
                if (!node.GetComponent<Node>().IsLeaf())
                {
                    if (m_Tree.TryGetValue(node.GetComponent<Node>().GetLeftChildID(), out var lc))
                    {
                        var edge = MenuEventManager.instance.MyInstantiate(m_SpringPrefab);
                        edge.GetComponent<Edge>().InitLineRenderer(node, lc, Edge.SpringType.heirarchal);

                        (var node1, var node2) = edge.GetComponent<Edge>().GetNodes();
                        ((nuint, nuint), (nuint, nuint)) edgeKey = (node1.GetComponent<Node>().GetId(), node2.GetComponent<Node>().GetId());

                        if (!m_EdgeCache.ContainsKey(edgeKey))
                        {
                            m_EdgeCache[edgeKey] = edge.gameObject;
                        }
                        else
                        {
                            Debug.LogWarning("Duplicate edge key found: " + edgeKey);
                            Destroy(edge.gameObject);
                        }

                    }

                    if (m_Tree.TryGetValue(node.GetComponent<Node>().GetRightChildID(), out var rc))
                    {
                        var edge = MenuEventManager.instance.MyInstantiate(m_SpringPrefab);
                        edge.GetComponent<Edge>().InitLineRenderer(node, rc, Edge.SpringType.heirarchal);

                        (var node1, var node2) = edge.GetComponent<Edge>().GetNodes();
                        ((nuint, nuint), (nuint, nuint)) edgeKey = (node1.GetComponent<Node>().GetId(), node2.GetComponent<Node>().GetId());

                        if (!m_EdgeCache.ContainsKey(edgeKey))
                        {
                            m_EdgeCache[edgeKey] = edge.gameObject;
                        }
                        else
                        {
                            Debug.LogWarning("Duplicate edge key found: " + edgeKey);
                        }

                    }
                }
            }
        }
        public Dictionary<(nuint, nuint), GameObject> GetTree()
        {
            return m_Tree;
        }

        public Dictionary<((nuint, nuint), (nuint, nuint)), GameObject> GetEdges()
        {
            return m_EdgeCache;
        }

        public void Set(Dictionary<(nuint, nuint), GameObject> tree)
        {
            m_Tree = tree;
        }

        public bool Contains((nuint, nuint) id)
        {
            return m_Tree.ContainsKey(id);
        }

        public GameObject Add((nuint, nuint) id)
        {
            //var wrapper = new RustResourceWrapper<ClusterIDs>(ClusterIDs.Alloc(id));
            (FFIError err, ClusterData cluster) = NativeMethods.GetClusterData(id);


            if (err == FFIError.Ok)
            {
                GameObject node = Instantiate(m_NodePrefab);
                node.GetComponent<Node>().SetID(cluster.ID_AsTuple());
                node.GetComponent<Node>().SetLeft(cluster.ID_AsTuple());
                node.GetComponent<Node>().SetRight(cluster.ID_AsTuple());
                m_Tree.Add(id, node);
                return node;
            }
            return null;
        }

        public GameObject GetOrAdd((nuint, nuint) id)
        {
            if (m_Tree.ContainsKey(id))
            {
                return m_Tree.GetValueOrDefault(id);
            }

            (FFIError err, ClusterData cluster) = NativeMethods.GetClusterData(id);

            //var wrapper = new RustResourceWrapper<ClusterIDs>(ClusterIDs.Alloc(id));
            if (err == FFIError.Ok)
            {
                GameObject node = Instantiate(m_NodePrefab);
                node.GetComponent<Node>().SetID(cluster.ID_AsTuple());
                node.GetComponent<Node>().SetLeft(cluster.ID_AsTuple());
                node.GetComponent<Node>().SetRight(cluster.ID_AsTuple());
                m_Tree.Add(id, node);
                return node;
            }
            else
            {
                Debug.Log("Get or add");
                Debug.Log(id.ToString());
                Debug.LogError(err.ToString());
                return null;
            }
        }

        bool SetVisibleTreeDepth(int maxDepth)
        {
            foreach (var kvp in m_Tree.ToList())
            {
                var cluster = kvp.Value;
                //var wrapper = new RustResourceWrapper<ClusterData>(ClusterData.Alloc(kvp.Key));
                (FFIError err, ClusterData clusterData) = NativeMethods.GetClusterData(kvp.Key);
                if (err == FFIError.Ok)

                {
                    if (clusterData.depth > maxDepth)
                    {
                        cluster.GetComponent<Node>().Deselect();
                        cluster.SetActive(false);
                    }
                }
            }
            return true;
        }

        public void Update()
        {
        }
        unsafe void SetNodeNames(ref Clam.FFI.ClusterIDs nodeData)
        {
            if (!m_Tree.ContainsKey(nodeData.id.AsTuple()))
            {
                GameObject node = Instantiate(m_NodePrefab);
                node.GetComponent<Node>().SetID(nodeData.id.AsTuple());
                node.GetComponent<Node>().SetLeft(nodeData.leftID.AsTuple());
                node.GetComponent<Node>().SetRight(nodeData.rightID.AsTuple());
                m_Tree.Add(nodeData.id.AsTuple(), node);
            }
        }

        unsafe void PositionUpdater(ref Clam.FFI.ClusterData nodeData)
        {
            if (m_Tree.TryGetValue(nodeData.ID_AsTuple(), out var node))
            {
                node.GetComponent<Node>().SetPosition(nodeData.pos.AsVector3);
            }
            else
            {
                Debug.Log("reingoldify key not found - " + nodeData.ID_AsString());
            }
        }

        unsafe void ColorFiller(ref Clam.FFI.ClusterData nodeData)
        {
            GameObject node;

            bool hasValue = m_Tree.TryGetValue(nodeData.ID_AsTuple(), out node);
            if (hasValue)
            {
                node.GetComponent<Node>().SetColor(nodeData.color.AsColor);
            }
            else
            {
                Debug.Log("cluster key not found - color filler - " + nodeData.ID_AsString());
            }
        }
    }
}
