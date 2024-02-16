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

        private Dictionary<string, GameObject> m_Tree;
        private Dictionary<string, GameObject> m_EdgeCache;
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

            m_Tree = new Dictionary<string, GameObject>();
            m_EdgeCache = new Dictionary<string, GameObject>();

            FFIError e = Clam.FFI.NativeMethods.SetNames(SetNodeNames);

            if (e == FFIError.Ok)
            {
                Debug.Log("ok)");
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


            Clam.FFI.NativeMethods.ForEachDFT(EdgeDrawer);
            PopulateEdgeDictionary();

            if (Clam.FFI.NativeMethods.GetRootData(out var rootData) == FFIError.Ok)
            {
                Debug.Log(System.String.Format("created tree with num nodes {0}.", rootData.Data.cardinality));
            }
            else
            {
                Debug.LogError("root not found?");
                return FFIError.HandleInitFailed;
            }

            return FFIError.Ok;
        }

        private void PopulateEdgeDictionary()
        {
            foreach((var id, var cluster) in m_Tree)
            {
                if (!cluster.GetComponent<Node>().IsLeaf())
                {
                   
                }
                
            }

            //Debug.Log("Populting edge keya");
            //m_EdgeCache = new Dictionary<string, GameObject>();
            //Edge[] edges = GameObject.FindObjectsOfType<Edge>(true);
            //foreach (Edge edge in edges)
            //{
            //    (var node1, var node2) = edge.GetComponent<Edge>().GetNodes();
            //    string edgeKey = node1.GetComponent<Node>().GetId() + node2.GetComponent<Node>().GetId();

            //    if (!m_EdgeCache.ContainsKey(edgeKey))
            //    {
            //        m_EdgeCache[edgeKey] = edge.gameObject;
            //    }
            //    else
            //    {
            //        Debug.LogWarning("Duplicate edge key found: " + edgeKey);
            //    }
            //}
        }

        public void ResetTree()
        {
            FFIError e = Clam.FFI.NativeMethods.SetNames(SetNodeNames);

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
            Clam.FFI.NativeMethods.ColorClustersByEntropy(ColorFiller);
            Clam.FFI.NativeMethods.ForEachDFT(EdgeDrawer);
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
            m_EdgeCache.Clear();
            m_EdgeCache = new Dictionary<string, GameObject>();
        }

        public void EdgeDrawer(ref FFI.ClusterData nodeData)
        {
            if (m_Tree.TryGetValue(nodeData.id.AsString, out var node))
            {
                if (!node.GetComponent<Node>().IsLeaf())
                {
                    if (m_Tree.TryGetValue(node.GetComponent<Node>().GetLeftChildID(), out var lc))
                    {
                        var edge = MenuEventManager.instance.MyInstantiate(m_SpringPrefab);
                        edge.GetComponent<Edge>().InitLineRenderer(node, lc, Edge.SpringType.heirarchal);

                        (var node1, var node2) = edge.GetComponent<Edge>().GetNodes();
                        string edgeKey = node1.GetComponent<Node>().GetId() + node2.GetComponent<Node>().GetId();

                        if (!m_EdgeCache.ContainsKey(edgeKey))
                        {
                            m_EdgeCache[edgeKey] = edge.gameObject;
                        }
                        else
                        {
                            Debug.LogWarning("Duplicate edge key found: " + edgeKey);
                        }

                    }

                    if (m_Tree.TryGetValue(node.GetComponent<Node>().GetRightChildID(), out var rc))
                    {
                        var edge = MenuEventManager.instance.MyInstantiate(m_SpringPrefab);
                        edge.GetComponent<Edge>().InitLineRenderer(node, rc, Edge.SpringType.heirarchal);

                        (var node1, var node2) = edge.GetComponent<Edge>().GetNodes();
                        string edgeKey = node1.GetComponent<Node>().GetId() + node2.GetComponent<Node>().GetId();

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
        public Dictionary<string, GameObject> GetTree()
        {
            return m_Tree;
        }

        public Dictionary<string, GameObject> GetEdges()
        {
            return m_EdgeCache;
        }

        public void Set(Dictionary<string, GameObject> tree)
        {
            m_Tree = tree;
        }

        public bool Contains(string id)
        {
            return m_Tree.ContainsKey(id);
        }

        public GameObject Add(string id)
        {
            var wrapper = new RustResourceWrapper<ClusterIDs>(ClusterIDs.Alloc(id));

            if (wrapper.result == FFIError.Ok)
            {
                GameObject node = Instantiate(m_NodePrefab);
                node.GetComponent<Node>().SetID(wrapper.Data.id.AsString);
                node.GetComponent<Node>().SetLeft(wrapper.Data.leftID.AsString);
                node.GetComponent<Node>().SetRight(wrapper.Data.rightID.AsString);
                m_Tree.Add(id, node);
                return node;
            }
            return null;
        }

        public GameObject GetOrAdd(string id)
        {
            if (m_Tree.ContainsKey(id))
            {
                return m_Tree.GetValueOrDefault(id);
            }
            var wrapper = new RustResourceWrapper<ClusterIDs>(ClusterIDs.Alloc(id));
            if (wrapper.result == FFIError.Ok)
            {
                GameObject node = Instantiate(m_NodePrefab);
                node.GetComponent<Node>().SetID(wrapper.Data.id.AsString);
                node.GetComponent<Node>().SetLeft(wrapper.Data.leftID.AsString);
                node.GetComponent<Node>().SetRight(wrapper.Data.rightID.AsString);
                m_Tree.Add(id, node);
                return node;
            }
            else
            {
                return null;
            }
        }

        bool SetVisibleTreeDepth(int maxDepth)
        {
            foreach (var kvp in m_Tree.ToList())
            {
                var cluster = kvp.Value;
                var wrapper = new RustResourceWrapper<ClusterData>(ClusterData.Alloc(kvp.Key));
                if (wrapper.result == FFIError.Ok)
                {
                    if (wrapper.Data.depth > maxDepth)
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
            if (!m_Tree.ContainsKey(nodeData.id.AsString))
            {
                GameObject node = Instantiate(m_NodePrefab);
                node.GetComponent<Node>().SetID(nodeData.id.AsString);
                node.GetComponent<Node>().SetLeft(nodeData.leftID.AsString);
                node.GetComponent<Node>().SetRight(nodeData.rightID.AsString);
                m_Tree.Add(nodeData.id.AsString, node);
            }
        }

        unsafe void PositionUpdater(ref Clam.FFI.ClusterData nodeData)
        {
            if (m_Tree.TryGetValue(nodeData.id.AsString, out var node))
            {
                node.GetComponent<Node>().SetPosition(nodeData.pos.AsVector3);
            }
            else
            {
                Debug.Log("reingoldify key not found - " + nodeData.id);
            }
        }

        unsafe void ColorFiller(ref Clam.FFI.ClusterData nodeData)
        {
            GameObject node;

            bool hasValue = m_Tree.TryGetValue(nodeData.id.AsString, out node);
            if (hasValue)
            {
                node.GetComponent<Node>().SetColor(nodeData.color.AsColor);
            }
            else
            {
                Debug.Log("cluster key not found - color filler - " + nodeData.id);
            }
        }
    }
}
