using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using Unity.VisualScripting;
using UnityEngine;
using UnityEngine.UIElements;

namespace Clam
{
    public class Cakes : MonoBehaviour
    {
        private static GameObject m_NodePrefab;
        private static GameObject m_SpringPrefab;

        private static Cakes instance;
        //private static Tree m_Tree;
        //public static TreeStartupData m_StartupData;
        //FFIError m_InitResult;
        private static bool m_Initialized = false;
        //private static IntPtr m_Handle;

        static public void BuildGraphWithSelected()
        {
            Dictionary<string, GameObject> graph = new Dictionary<string, GameObject>();

            foreach(var (id, node) in Tree.GetTree())
            {
                if (node.activeSelf && node.GetComponent<Node>().IsSelected())
                {
                    graph[id] = node;
                }
                else
                {
                    Destroy(node);
                    //node.SetActive(false);
                }
            }

            Debug.Log("building graph with size" + graph.Count);
            //var graph = Tree.GetTree().Where(
            //    kvp => kvp.Value.activeSelf && kvp.Value.GetComponent<Node>().IsSelected())
            //    .ToDictionary(kvp => kvp.Key, kvp => kvp.Value); 

            Tree.Set(graph);
        }

        static public void BuildGraphWithinParams()
        {
            Dictionary<string, GameObject> graph = new Dictionary<string, GameObject>();

            foreach (var (id, node) in Tree.GetTree())
            {
                if (node.GetComponent<Node>().IsSelected())
                {
                    if (!node.activeSelf)
                    {
                        node.SetActive(true);
                    }
                    graph[id] = node;
                }
                else
                {
                    Destroy(node);
                    //node.SetActive(false);
                }
            }

            Debug.Log("building graph with size" + graph.Count);
            //var graph = Tree.GetTree().Where(
            //    kvp => kvp.Value.activeSelf && kvp.Value.GetComponent<Node>().IsSelected())
            //    .ToDictionary(kvp => kvp.Key, kvp => kvp.Value); 

            Tree.Set(graph);
        }


        private static Cakes Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = FindObjectOfType<Cakes>();
                    if (instance == null)
                    {
                        GameObject obj = new GameObject();
                        obj.name = typeof(Cakes).Name;
                        instance = obj.AddComponent<Cakes>();
                        //m_Handle = FFI.NativeMethods.GetHandle();
                        //m_Tree = obj.AddComponent<Tree>();
                    }


                }
                if (instance.GetComponent<TreeCache>() == null)
                {
                    Debug.LogWarning("tree not added yet in instance");
                    InitTree();

                }
                return instance;
            }
        }

        public static TreeCache Tree
        {
            get
            {
                return Instance.GetComponent<TreeCache>();
            }
        }

        protected virtual void Awake()
        {
            if (instance == null)
            {
                //instance = this as Cakes;
                instance = FindObjectOfType<Cakes>();
                InitTree();
                //if (instance.GetComponent<Tree>() == null)
                //{
                //    Debug.LogWarning("tree not added yet in instance");
                //    InitTree();

                //}
                //InitTree();
                //DontDestroyOnLoad(gameObject);
            }
            else
            {
                Destroy(gameObject);
                //Debug.LogWarning("Why is destroy here?...");
            }
        }

        void OnDestroy()
        {
            Debug.Log("OnDestroy cakes");

            //Debug.Log("Application ending after " + Time.time + " seconds");
            //m_Tree = new Dictionary<string, GameObject>();
            //m_SelectedNode = null;
            //if (m_InitResult == FFIError.Ok)
            if (m_Initialized)
            {
                Clam.FFI.NativeMethods.ForceShutdownPhysics();
                Clam.FFI.NativeMethods.ShutdownClam();
                //m_InitResult = FFIError.HandleInitFailed;
            }
        }

        private static void InitTree()
        {
            TreeCache tree = instance.AddComponent<TreeCache>();
            m_SpringPrefab = Resources.Load("Spring") as GameObject;
            m_NodePrefab = Resources.Load("Node") as GameObject;


            FFIError initResult = tree.Init(m_NodePrefab, m_SpringPrefab);

            if (initResult == FFIError.Ok)
            {
                m_Initialized = true;
            }
            else
            {
                Debug.LogError("Tree initializtion failed with error " + initResult);
#if UNITY_EDITOR
                // Application.Quit() does not work in the editor so
                // UnityEditor.EditorApplication.isPlaying need to be set to false to end the game
                UnityEditor.EditorApplication.isPlaying = false;
#else
        Application.Quit();
#endif
            }
        }

        void OnApplicationQuit()
        {
            Debug.Log("Application ending after " + Time.time + " seconds");
            
        }
    }
}

