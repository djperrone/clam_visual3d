
#pragma warning disable CS8500
#pragma warning disable CS8981
using Clam;
using System;
using System.Collections.Generic;
using System.Text;
using UnityEngine;
using UnityEngine.UIElements;

namespace Clam
{
    namespace FFI
    {
        public unsafe delegate void NodeVisitor(ref Clam.FFI.ClusterData baton);
        public unsafe delegate void NameSetter(ref Clam.FFI.ClusterIDs baton);
        public unsafe delegate void NodeVisitorMut(ref Clam.FFI.ClusterData inData);

        public static partial class NativeMethods
        {
	public const string __DllName = "clam_ffi_2024-03-0212-37-47";
            private static IntPtr m_Handle;

            private static bool m_Initialized = false;

            // init/shutdown functions for clam
            public static FFIError InitClam(string dataName, uint cardinality, DistanceMetric distanceMetric)
            {
                byte[] byteName = Encoding.UTF8.GetBytes(dataName);
                int len = byteName.Length;
                var e = init_clam(out m_Handle, byteName, len, cardinality, distanceMetric);
                if (e == FFIError.Ok)
                {
                    m_Initialized = true;
                }
                return e;
            }

            public static FFIError InitClam(TreeStartupData data)
            {
                var wrapper = new RustResourceWrapper<TreeStartupDataFFI>(TreeStartupDataFFI.Alloc(data));

                var refData = wrapper.GetData();
                var e = init_clam(out m_Handle, ref refData);
                if (e == FFIError.Ok)
                {
                    m_Initialized = true;
                }
                return e;
            }

            public static FFIError InitClamGraph(ScoringFunction scoringFunction, int minDepth, NodeVisitor clusterSelector)
            {
                var e = init_clam_graph(m_Handle, scoringFunction, minDepth, clusterSelector);
                if (e == FFIError.Ok)
                {
                    m_Initialized = true;
                }
                return e;
            }

            public static FFIError ShutdownClam()
            {
                if (m_Handle != IntPtr.Zero && m_Initialized)
                {
                    var e = shutdown_clam(out m_Handle);
                    m_Initialized = false;
                    m_Handle = IntPtr.Zero;
                    return e;
                }
                Debug.Log("Failed to shutdown clam");
                return FFIError.NullPointerPassed;
            }

            // -------------------------------------  File IO ------------------------------------- 

            public static FFIError SaveCakes(string dataName)
            {
                byte[] byteName = Encoding.UTF8.GetBytes(dataName);
                int len = byteName.Length;
                var e = save_cakes(m_Handle, byteName, len);
                if (e == FFIError.Ok)
                {
                    m_Initialized = true;
                }
                return e;
            }

            public static FFIError LoadCakes(TreeStartupData data)
            {
                var wrapper = new RustResourceWrapper<TreeStartupDataFFI>(TreeStartupDataFFI.Alloc(data));
                if (wrapper.result == FFIError.Ok)
                {
                    var refData = wrapper.GetData();
                    var e = load_cakes(out m_Handle, ref refData);
                    if (e == FFIError.Ok)
                    {
                        m_Initialized = true;
                    }
                    return e;
                }
                else
                {
                    return wrapper.result;
                }
            }

            // -------------------------------------  Tree helpers ------------------------------------- 

            public static FFIError ForEachDFT(NodeVisitor callback, string startNode = "root", int maxDepth = -1)
            {
                if (maxDepth == -1)
                {
                    return for_each_dft(m_Handle, callback, startNode, NativeMethods.TreeHeight());
                }
                else
                {
                    return for_each_dft(m_Handle, callback, startNode, maxDepth);
                }
            }

            public static int GetClusterLabel(string id)
            {
                return get_cluster_label(m_Handle, id);
            }

            public static FFIError SetNames(NameSetter callback, string startNode = "root")
            {
                return set_names(m_Handle, callback, startNode);
            }

            public static int TreeHeight()
            {
                return tree_height(m_Handle);
            }

            public static int VertexDegree(string clusterID)
            {
                return vertex_degree(m_Handle, clusterID);
            }

            public static int MaxVertexDegree()
            {
                return max_vertex_degree(m_Handle);
            }

            public static float MaxLFD()
            {
                return max_lfd(m_Handle);
            }
            public static int TreeCardinality()
            {
                return tree_cardinality(m_Handle);
            }

            public static FFIError ColorClustersByEntropy(NodeVisitor callback)
            {
                return color_clusters_by_entropy(m_Handle, callback);
            }
            public static FFIError ColorClustersByDominantLabel(NodeVisitor callback)
            {
                return color_clusters_by_dominant_label(m_Handle, callback);
            }

            // ------------------------------------- Cluster Helpers ------------------------------------- 

            public static FFIError AllocString(string data, out StringFFI resource)
            {
                var result = alloc_string(data, out resource);
                return result;
            }

            public static FFIError CreateClusterIDsMustFree(string id, out Clam.FFI.ClusterIDs clusterData)
            {
                var result = create_cluster_ids(m_Handle, id, out var data);
                if (result != FFIError.Ok)
                {
                    clusterData = new ClusterIDs();
                }
                clusterData = data;
                return FFIError.Ok;
            }

            public static FFIError CreateClusterDataMustFree(string id, out Clam.FFI.ClusterData clusterData, bool addIfNotExists = false)
            {
                var result = create_cluster_data(m_Handle, id, out var data);
                if (result != FFIError.Ok)
                {
                    clusterData = new ClusterData();
                    return result;
                }
                if (Cakes.Tree.GetTree().TryGetValue(data.id.AsString, out var node))
                {
                    data.SetPos(node.GetComponent<Node>().GetPosition());
                    data.SetColor(node.GetComponent<Node>().GetColor());
                    clusterData = data;
                    return FFIError.Ok;
                }
                else
                {
                    clusterData = data;
                    return FFIError.NotInCache;
                }
            }

            public static FFIError DeleteClusterData(ref ClusterData data)
            {
                return delete_cluster_data(ref data, out var outData);
            }

            public static FFIError FreeString(ref StringFFI data)
            {
                return free_string(ref data, out var outData);
            }

            public static FFIError DeleteClusterIDs(ref ClusterIDs data)
            {
                return delete_cluster_ids(ref data, out var outData);
            }

            public static FFIError SetMessage(string msg, out ClusterData data)
            {
                set_message(msg, out data);
                return FFIError.Ok;
            }

            public static FFIError GetRootData(out RustResourceWrapper<ClusterData> clusterDataWrapper)
            {
                string rootID = "0-" + NativeMethods.TreeCardinality().ToString();

                clusterDataWrapper = new RustResourceWrapper<ClusterData>(ClusterData.Alloc(rootID));

                if (clusterDataWrapper.result == FFIError.Ok || clusterDataWrapper.result == FFIError.NotInCache)
                {
                    return clusterDataWrapper.result;
                }

                clusterDataWrapper = null;
                return FFIError.InvalidStringPassed;
            }

            public static unsafe float DistanceToOther(string node1, string node2)
            {
                return distance_to_other(m_Handle, node1, node2); ;
            }

            // Reingold Tilford Tree Layout
            public static FFIError DrawHierarchy(NodeVisitor callback)
            {
                return draw_hierarchy(m_Handle, callback);
            }

            public static FFIError DrawHierarchyOffsetFrom(RustResourceWrapper<ClusterData> wrapper, NodeVisitor callback, int rootDepth = 0, int currentDepth = 1, int maxDepth = 1)
            {
                ClusterData nodeData = wrapper.Data;
                return draw_hierarchy_offset_from(m_Handle, ref nodeData, currentDepth, maxDepth - rootDepth, callback);
            }

            // Graph Physics
            public static FFIError InitForceDirectedGraph(float scalar, int maxIters)
            {
                return init_force_directed_graph(m_Handle, scalar, maxIters);
            }
            public static void InitGraphVertices(NodeVisitorMut edgeCB)
            {
                init_graph_vertices(m_Handle, edgeCB);
            }

            public static void RunTriangleTest(bool lastRun, string outPath, NodeVisitorMut clusterGetter)
            {
                run_triangle_test(m_Handle, lastRun, outPath, clusterGetter);
            }

            public static FFIError PhysicsUpdateAsync(NodeVisitor cb_fn)
            {
                return physics_update_async(m_Handle, cb_fn);
            }

            //public static FFIError ShutdownPhysics()
            //{
            //    return shutdown_physics(m_Handle);
            //}
            public static FFIError ForceShutdownPhysics()
            {
                return force_physics_shutdown(m_Handle);
            }

            // -1 if no graph
            public static int GetNumGraphEdges()
            {
                return get_num_edges_in_graph(m_Handle);
            }

            // -1 if no graph
            public static int GetGraphClusterCardinality()
            {
                return get_graph_cluster_cardinality(m_Handle);
            }

            // -1 if no graph
            public static int GetNumGraphComponents()
            {
                return get_num_graph_components(m_Handle);
            }

            // RNN 
            public static int TestCakesRNNQuery(float searchRadius, NodeVisitor callback)
            {
                return test_cakes_rnn_query(m_Handle, searchRadius, callback);
            }
        }
    }
}
