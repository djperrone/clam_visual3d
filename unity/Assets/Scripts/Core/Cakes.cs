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
        private static bool m_Initialized = false;

        static public void BuildGraphWithSelected()
        {
            Dictionary<(nuint, nuint), GameObject> graph = new Dictionary<(nuint, nuint), GameObject>();

            foreach(var (id, node) in Tree.GetTree())
            {
                if (node.activeSelf && node.GetComponent<Node>().IsSelected())
                {
                    graph[id] = node;
                }
                else
                {
                    Destroy(node);
                }
            }

            Debug.Log("building graph with size" + graph.Count);
            Tree.Set(graph);
        }

        static public void BuildGraphWithinParams()
        {
            Dictionary<(nuint, nuint), GameObject> graph = new Dictionary<(nuint, nuint), GameObject>();

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
                }
            }

            Debug.Log("building graph with size" + graph.Count);
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
                instance = FindObjectOfType<Cakes>();
                InitTree();
            }
            else
            {
                Destroy(gameObject);
            }
        }

        void OnDestroy()
        {
            Debug.Log("OnDestroy cakes");

            if (m_Initialized)
            {
                Clam.FFI.NativeMethods.ForceShutdownPhysics();
                Clam.FFI.NativeMethods.ShutdownClam();
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

