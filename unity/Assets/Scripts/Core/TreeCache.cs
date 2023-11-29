using Clam;
using Clam.FFI;
using System.Collections;
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

        //private float m_EdgeScalar = 25.0f;
        //private float m_SearchRadius = 0.05f;
        public bool m_IsPhysicsRunning = false;

        //public void Init(GameObject nodePrefab, GameObject springPrefab, string dataName, uint cardinality)
        public FFIError Init(GameObject nodePrefab, GameObject springPrefab)
        {
            m_NodePrefab = nodePrefab;
            m_SpringPrefab = springPrefab;
            //m_NodePrefab = nodePrefab;
            //m_SpringPrefab = springPrefab;
            //m_DataName = dataName;
            //m_Cardinality = cardinality;
            //m_TreeData = new TreeStartupData();
            m_TreeData = MenuEventManager.instance.m_TreeData;
            //m_TreeData.dataName = "arrhythmia";
            //m_TreeData.cardinality = 15;
            if (m_TreeData.dataName == null || m_TreeData.dataName.Length == 0)
            {
                Debug.Log("error with tree data");
                //Application.Quit();
                return FFIError.StartupDataInvalid;

            }

            if (m_TreeData.shouldLoad)
            {
                FFIError clam_result = Clam.FFI.NativeMethods.LoadCakes(m_TreeData);
                //FFIError clam_result = Clam.FFI.NativeMethods.LoadCakes(m_TreeData.dataName);
                if (clam_result != FFIError.Ok)
                {
                    Debug.Log("error with tree data");
                    return clam_result;
                }
            }
            else
            {
                FFIError clam_result = Clam.FFI.NativeMethods.InitClam(m_TreeData);
                //FFIError clam_result = Clam.FFI.NativeMethods.InitClam(m_TreeData.dataName, m_TreeData.cardinality, m_TreeData.distanceMetric);
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
            Clam.FFI.NativeMethods.ColorClustersByLabel(ColorFiller);

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

            //SetVisibleTreeDepth(7);
            return FFIError.Ok;
        }

        private void PopulateEdgeDictionary()
        {
            m_EdgeCache = new Dictionary<string, GameObject>();
            Edge[] edges = GameObject.FindObjectsOfType<Edge>(true);
            foreach (Edge edge in edges)
            {
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
            Clam.FFI.NativeMethods.DrawHierarchy(PositionUpdater);
            Clam.FFI.NativeMethods.ColorClustersByLabel(ColorFiller);

            //Edge[] edges = GameObject.FindObjectsOfType<Edge>(true);
            var edges = GameObject.FindGameObjectsWithTag(m_SpringPrefab.tag);

            foreach (var obj in edges)
            {
                if (obj is GameObject)
                {
                    GameObject gameObject = obj as GameObject;
                    Destroy(gameObject); // or use DestroyImmediate(gameObject) if you want to destroy it immediately
                }
                // Add additional checks or actions for other types if needed
            }

            Clam.FFI.NativeMethods.ForEachDFT(EdgeDrawer);
            PopulateEdgeDictionary();
        }


        public void EdgeDrawer(ref FFI.ClusterData nodeData)
        {
            if (m_Tree.TryGetValue(nodeData.id.AsString, out var node))
            {
                if (node.activeSelf && !node.GetComponent<Node>().IsLeaf())
                {
                    if (m_Tree.TryGetValue(node.GetComponent<Node>().GetLeftChildID(), out var lc))
                    {
                        var spring = MenuEventManager.instance.MyInstantiate(m_SpringPrefab);
                        //var sprint = SpringScript.CreateInstance(node, lc, SpringScript.SpringType.heirarchal);
                        //var spring = MenuEventManager.instance.MyInstantiate(m_SpringPrefab);

                        spring.GetComponent<Edge>().InitLineRenderer(node, lc, Edge.SpringType.heirarchal);

                        //spring.GetComponent<SpringScript>().SetNodes(node, lc);
                        //spring.GetComponent<SpringScript>().SetColor(Color.white);

                    }

                    if (m_Tree.TryGetValue(node.GetComponent<Node>().GetRightChildID(), out var rc))
                    {
                        var spring = MenuEventManager.instance.MyInstantiate(m_SpringPrefab);

                        spring.GetComponent<Edge>().InitLineRenderer(node, rc, Edge.SpringType.heirarchal);

                        //spring.GetComponent<SpringScript>().SetNodes(node, rc);
                        //var sprint = SpringScript.CreateInstance(node, lc, SpringScript.SpringType.Similarity);

                        //spring.GetComponent<SpringScript>().SetColor(Color.white);
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

            //var wrapper = NativeMethods.CreateClusterIDsWrapper(id);
            if (wrapper.result == FFIError.Ok)
            {
                GameObject node = Instantiate(m_NodePrefab);
                //nodeData.LogInfo();
                node.GetComponent<Node>().SetID(wrapper.Data.id.AsString);
                node.GetComponent<Node>().SetLeft(wrapper.Data.leftID.AsString);
                node.GetComponent<Node>().SetRight(wrapper.Data.rightID.AsString);
                print("setting name here add" + node.GetComponent<Node>().GetId());
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
            //var wrapper = NativeMethods.CreateClusterIDsWrapper(id);
            if (wrapper.result == FFIError.Ok)
            {
                GameObject node = Instantiate(m_NodePrefab);
                node.GetComponent<Node>().SetID(wrapper.Data.id.AsString);
                node.GetComponent<Node>().SetLeft(wrapper.Data.leftID.AsString);
                node.GetComponent<Node>().SetRight(wrapper.Data.rightID.AsString);
                print("setting name here getoradd " + node.GetComponent<Node>().GetId());
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
                //if (!cluster.activeSelf)
                //{
                //    continue;
                //}
                //Clam.FFI.ClusterDataWrapper wrapper = Clam.FFI.NativeMethods.CreateClusterDataWrapper(kvp.Key);
                var wrapper = new RustResourceWrapper<ClusterData>(ClusterData.Alloc(kvp.Key));
                //Clam.FFI.ClusterDataWrapper wrapper = new Clam.FFI.ClusterDataWrapper(cluster.GetComponent<Node>().ToNodeData());
                //Clam.FFI.NativeMethods.GetClusterData(wrapper);
                if (wrapper.result == FFIError.Ok)
                {
                    //if (m_IntInputFields.TryGetValue("Depth", out var depthField))
                    {
                        //var range = textField.MinMaxRange();
                        if (wrapper.Data.depth > maxDepth)
                        {
                            cluster.GetComponent<Node>().Deselect();
                            cluster.SetActive(false);
                            //continue;
                        }
                    }


                }
                //cluster.GetComponent<Node>().Select();
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
                //nodeData.LogInfo();
                node.GetComponent<Node>().SetID(nodeData.id.AsString);
                node.GetComponent<Node>().SetLeft(nodeData.leftID.AsString);
                node.GetComponent<Node>().SetRight(nodeData.rightID.AsString);
                print("setting name here " + node.GetComponent<Node>().GetId());
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
                Debug.Log("setting color to" + nodeData.color.AsVector3.ToString());
                node.GetComponent<Node>().SetColor(nodeData.color.AsColor);
            }
            else
            {
                Debug.Log("cluster key not found - color filler - " + nodeData.id);
            }
        }
    }
}

