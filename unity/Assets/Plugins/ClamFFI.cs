// <auto-generated>
// This code is generated by csbindgen.
// DON'T CHANGE THIS DIRECTLY.
// </auto-generated>
#pragma warning disable CS8500
#pragma warning disable CS8981
using System;
using System.Collections.Generic;
//using System.Numerics;
using System.Runtime.InteropServices;
using System.Text;
using UnityEditor.Experimental.GraphView;
using UnityEngine;


namespace ClamFFI
{
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Vec3
    {
        public float x;
        public float y;
        public float z;

        public Vec3(float x, float y, float z)
        {
            this.x = x;
            this.y = y;
            this.z = z;
        }
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe class NodeBaton
    {
        public Vec3 pos;
        public Vec3 color;
        public byte* id;
        public byte* leftID;
        public byte* rightID;
        public int cardinality;
        public int depth;
        public int argCenter;
        public int argRadius;
        public int idLen;

        public NodeBaton(Node node)
        {
            byte[] byteName = Encoding.UTF8.GetBytes(node.id);
            int len = byteName.Length;

            unsafe
            {
                fixed (char* p = node.id)
                {
                    // do some work
                    //this.id = (byte*)p;
                    this.cardinality = node.cardinality;
                    this.depth = node.depth;
                    this.argCenter = node.argCenter;
                    this.argRadius = node.argRadius;
                    this.id = null;
                    this.leftID = null;
                    this.rightID = null;
                    this.pos = new Vec3();
                    this.color = new Vec3();
                    //this.idLen = node.id.Length;
                }
            }
            
        }
        public NodeBaton()
        {
           
            this.cardinality = -1;
            this.depth = -1;
            this.argCenter = -1;
            this.argRadius = -1;
            this.id = null;
            this.leftID = null;
            this.rightID = null;
            this.pos = new Vec3();
            this.color = new Vec3();
        }
    }
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public class NodeFromClam
    {
        //public Vec3 pos;
        public int cardinality;
        public int depth;
        public int argCenter;
        public int argRadius;
        public NodeFromClam()
        {
            this.cardinality = -1;
            this.depth = -1;
            this.argCenter = -1;
            this.argRadius = -1;
            //this.pos = new Vec3();
        }
    }

    public unsafe class Node
    {
        public Vector3 pos;
        public Color color;
        public string id, leftID, rightID;
        public int cardinality;
        public int depth;
        public int argCenter;
        public int argRadius;

        public Node(string id, string leftID, string rightID, Vector3 position, Color color)
        {
            this.id = id; this.leftID = leftID;
            this.rightID = rightID;
            this.pos = position;
            this.color = color;
            this.cardinality = -1;
            this.depth = -1;
            this.argCenter = -1;
            this.argRadius = -1;
        }

        public Node(NodeBaton baton)
        {
            pos = new Vector3(baton.pos.x, baton.pos.y, baton.pos.z);
            color = new Color(baton.color.x, baton.color.y, baton.color.z);
            //Debug.Log("node constructor");
            if (baton.id != null)
            {
                //Debug.Log("setting id");

                id = new String((sbyte*)baton.id);
            }
            else
            {
                Debug.Log("id null");

                id = "default";
            }
            if (baton.leftID != null)
            {
                //Debug.Log("setting leftID");

                leftID = new String((sbyte*)baton.leftID);

            }
            else
            {
                //Debug.Log("leftID null");

                leftID = "default";
            }
            if (baton.rightID != null)
            {
                // Debug.Log("setting rightID");

                rightID = new String((sbyte*)baton.rightID);

            }
            else
            {
                //Debug.Log("rightID null");

                rightID = "default";
            }

            // Debug.Log("setting clam params");

            cardinality = baton.cardinality;
            depth = baton.depth;
            argCenter = baton.argCenter;
            argRadius = baton.argRadius;

            // Debug.Log("constructor done");


        }

        public Node(NodeBaton2 baton)
        {
            pos = new Vector3(baton.pos.x, baton.pos.y, baton.pos.z);
            color = new Color(baton.color.x, baton.color.y, baton.color.z);
            //Debug.Log("node constructor");
            if (baton.id != null)
            {
                //Debug.Log("setting id");

                id = new String((sbyte*)baton.id);
            }
            else
            {
                Debug.Log("id null");

                id = "default";
            }
            if (baton.leftID != null)
            {
                //Debug.Log("setting leftID");

                leftID = new String((sbyte*)baton.leftID);

            }
            else
            {
                //Debug.Log("leftID null");

                leftID = "default";
            }
            if (baton.rightID != null)
            {
                // Debug.Log("setting rightID");

                rightID = new String((sbyte*)baton.rightID);

            }
            else
            {
                //Debug.Log("rightID null");

                rightID = "default";
            }

            // Debug.Log("setting clam params");

            cardinality = baton.cardinality;
            depth = baton.depth;
            argCenter = baton.argCenter;
            argRadius = baton.argRadius;

            // Debug.Log("constructor done");


        }

        //public Node(string id, Vector3 pos, Color color)
        //{
        //    this.id = id;
        //    this.pos = pos;
        //    this.color = color;
        //}
    }
 

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public unsafe partial struct NodeBaton2
    {
        public Vec3 pos;
        public Vec3 color;
        public byte* id;
        public byte* leftID;
        public byte* rightID;

        public int cardinality;
        public int depth;
        public int argCenter;
        public int argRadius;

        public NodeBaton2(Node node)
        {
            this.cardinality = node.cardinality;
            this.depth = node.depth;
            this.argCenter = node.argCenter;
            this.argRadius = node.argRadius;
            this.id = null;
            this.leftID = null;
            this.rightID = null;
            this.pos = new Vec3(node.pos.x, node.pos.y, node.pos.z);
            this.color = new Vec3(node.color.r, node.color.g, node.color.b);
        }
        public NodeBaton2(int test)
        {
            this.cardinality = -1;
            this.depth = -1;
            this.argCenter = -1;
            this.argRadius = -1;
            this.id = null;
            this.leftID = null;
            this.rightID = null;
            this.pos = new Vec3();
            this.color = new Vec3();
        }
    }

    public static class Clam
    {
	public const string __DllName = "clam_ffi_20230615161850";
        private static IntPtr _handle;

        public unsafe delegate void NodeVisitor(NodeBaton baton);


        [DllImport(__DllName, CallingConvention = CallingConvention.Cdecl, EntryPoint = "example_double_super_complex_entity")]
        public static extern void example_double_super_complex_entity(IntPtr context, NodeFromClam incoming, out NodeFromClam outgoing);

        public static NodeFromClam ExampleDoubleEtc(ref NodeFromClam incoming)
        {
            example_double_super_complex_entity(_handle, incoming, out var outgoing);
            Debug.Log("finished c# example call");
            return incoming;
        }

        [DllImport(__DllName, CallingConvention = CallingConvention.Cdecl, EntryPoint = "free_string")]
        public unsafe static extern void free_string(IntPtr context, byte* data);

        public unsafe static void FreeString(byte* data)
        {
            free_string(_handle, data);
            Debug.Log("finishedfreeing");
        }


        [DllImport(__DllName, EntryPoint = "get_node_data2", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static unsafe extern void get_node_data2(IntPtr handle, ref NodeBaton2 inNode, out NodeBaton2 outNode);

        public static unsafe Node GetNodeData2(string name)
        {
            //byte[] byteName = Encoding.UTF8.GetBytes(name);
            //int len = byteName.Length;
            NodeBaton2 baton = new NodeBaton2(1);
            //Debug.Log("card here3 " + baton.cardinality);
            //Debug.Log("test here ");

            get_node_data2(_handle, ref baton, out var outNode);
            //Debug.Log("test here ");

            Debug.Log("card here " + outNode.cardinality);
            Debug.Log("argr here " + outNode.argRadius);
            Debug.Log("argc here " + outNode.argCenter);
            Debug.Log("depth here " + outNode.depth);
            Node finalNode = new Node(outNode);
            free_node_string(_handle, ref baton, out var freedBaton);
            //FreeString(baton.leftID);
            //FreeString(baton.rightID);
            Debug.Log("card here2 " + baton.cardinality);
            return finalNode;
        }


        [DllImport(__DllName, EntryPoint = "get_node_data3", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static unsafe extern void get_node_data3(IntPtr handle, byte[] binary_id, int idLen, ref NodeBaton2 inNode, out NodeBaton2 outNode);

        public static unsafe Node GetNodeData3(ClamFFI.Node nodeData)
        {
           
            NodeBaton2 baton = new NodeBaton2(1);
            string binaryID = HexStringToBinary(nodeData.id);
            binaryID.TrimStart('0');
            byte[] byteName = Encoding.UTF8.GetBytes(binaryID);
            int len = byteName.Length;
            Debug.Log("bytename " + binaryID);
           
            get_node_data3(_handle, byteName, byteName.Length, ref baton, out var outNode);

            Debug.Log("depth here " + outNode.depth);
            Node finalNode = new Node(outNode);
            free_node_string(_handle, ref baton, out var freedBaton);
            return finalNode;
        }

        [DllImport(__DllName, EntryPoint = "free_node_string", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static unsafe extern void free_node_string(IntPtr handle, ref NodeBaton2 inNode, out NodeBaton2 outNode);


        [DllImport(__DllName, EntryPoint = "destroy_node_baton", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static unsafe extern void destroy_node_baton(NodeBaton context);


        [DllImport(__DllName, EntryPoint = "create_reingold_layout", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern int create_reingold_layout(IntPtr ptr, NodeVisitor callback);

        public static int CreateReingoldLayout(NodeVisitor callback)
        {
            return create_reingold_layout(_handle, callback);
        }

        [DllImport(__DllName, EntryPoint = "get_num_nodes", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern int get_num_nodes(IntPtr handle);

        public static int GetNumNodes()
        {
            return get_num_nodes(_handle);
        }

        [DllImport(__DllName, EntryPoint = "test", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern int test();

        public static int Test()
        {
            return test();
        }

        [DllImport(__DllName, EntryPoint = "traverse_tree_df", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern int traverse_tree_df(IntPtr ptr, NodeVisitor callback);

        public static int TraverseTreeDF(NodeVisitor callback)
        {
            return traverse_tree_df(_handle, callback);
        }

        [DllImport(__DllName, EntryPoint = "init_clam", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern int init_clam(out IntPtr ptr, byte[] data_name, int name_len, uint cardinality);

        public static int InitClam(string dataName, uint cardinality)
        {
            byte[] byteName = Encoding.UTF8.GetBytes(dataName);
            int len = byteName.Length;

            return init_clam(out _handle, byteName, len, cardinality);
        }

        private static readonly Dictionary<char, string> hexCharacterToBinary = new Dictionary<char, string> {
        { '0', "0000" },
        { '1', "0001" },
        { '2', "0010" },
        { '3', "0011" },
        { '4', "0100" },
        { '5', "0101" },
        { '6', "0110" },
        { '7', "0111" },
        { '8', "1000" },
        { '9', "1001" },
        { 'a', "1010" },
        { 'b', "1011" },
        { 'c', "1100" },
        { 'd', "1101" },
        { 'e', "1110" },
        { 'f', "1111" }
    };

        public static string HexStringToBinary(string hex)
        {
            StringBuilder result = new StringBuilder();
            foreach (char c in hex)
            {
                // This will crash for non-hex characters. You might want to handle that differently.
                result.Append(hexCharacterToBinary[char.ToLower(c)]);
            }
            return result.ToString();
        }
    }


}
