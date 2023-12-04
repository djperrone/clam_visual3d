using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using UnityEngine;

namespace Clam
{
    namespace FFI
    {

        //public class ClusterDataWrapper
        //{
        //    private ClusterData m_Data;

        //    public ClusterDataWrapper(ClusterData nodeData)
        //    {
        //        m_Data = nodeData;
        //    }

        //    public ClusterData Data
        //    {
        //        get { return m_Data; }
        //        set
        //        {
        //            //m_Data.FreeStrings();
        //            m_Data = value;
        //        }
        //    }

        //    ~ClusterDataWrapper()
        //    {
        //        //m_Data.FreeStrings();
        //        //var data = wrapper.Data;
        //        Clam.FFI.NativeMethods.DeleteClusterData(ref m_Data);
        //    }
        //}

        [Serializable]
        [StructLayout(LayoutKind.Sequential)]
        public partial struct ClusterData : IRustResource
        {
            public Vec3 pos;
            public Vec3 color;
            public StringFFI id;
            //public StringFFI leftID;
            //public StringFFI rightID;
            // a string storing 100 bytes for passing misc values between rust and unity - allocated by rust
            public StringFFI message;
            public int cardinality;
            public int depth;
            public float radius;
            public float lfd;
            public int argCenter;
            public int argRadius;

            public int vertexDegree;

            public float distToQuery;

            public void SetPos(Vector3 pos)
            {
                this.pos.x = pos.x;
                this.pos.y = pos.y;
                this.pos.z = pos.z;
            }
            public void SetColor(Color color)
            {
                this.color.x = color.r;
                this.color.y = color.g;
                this.color.z = color.b;
            }

            public static (ClusterData, FFIError) Alloc(string data)
            {
                var result = NativeMethods.CreateClusterDataMustFree(data, out var resource);
                return (resource, result);
            }

            public void LogInfo()
            {
                Debug.Log("id: " + this.id.AsString);
                Debug.Log("pos: " + this.pos.AsVector3);
                Debug.Log("color: " + this.color.AsColor);

                //Debug.Log("leftID: " + this.leftID.AsString);
                //Debug.Log("rightID: " + this.rightID.AsString);
                Debug.Log("depth: " + this.depth);
                Debug.Log("cardinality: " + this.cardinality);
                Debug.Log("argCenter: " + this.argCenter);
                Debug.Log("argRadius: " + this.argRadius);
            }

            public string GetInfo()
            {
                StringBuilder stringBuilder = new StringBuilder();

                stringBuilder.AppendLine("id: " + this.id.AsString);
                //stringBuilder.AppendLine("leftID: " + this.leftID.AsString);
                //stringBuilder.AppendLine("rightID: " + this.rightID.AsString);
                stringBuilder.AppendLine("depth " + depth.ToString());
                stringBuilder.AppendLine("card: " + cardinality.ToString());
                stringBuilder.AppendLine("radius: " + radius.ToString());
                stringBuilder.AppendLine("lfd: " + lfd.ToString());
                stringBuilder.AppendLine("argC: " + argCenter.ToString());
                stringBuilder.AppendLine("argR: " + argRadius.ToString());
                //stringBuilder.AppendLine(this.color.ToString());

                return stringBuilder.ToString();
            }
            public string GetInfoForUI()
            {
                StringBuilder stringBuilder = new StringBuilder();

                stringBuilder.AppendLine(this.id.AsString);
                stringBuilder.AppendLine(depth.ToString());
                stringBuilder.AppendLine(cardinality.ToString());
                stringBuilder.AppendLine(radius.ToString());
                stringBuilder.AppendLine(lfd.ToString());
                stringBuilder.AppendLine(argCenter.ToString());
                stringBuilder.AppendLine(argRadius.ToString());

                return stringBuilder.ToString();

            }
            public void Free()
            {
                Clam.FFI.NativeMethods.DeleteClusterData(ref this);
            }
        }

        [Serializable]
        [StructLayout(LayoutKind.Sequential)]
        public partial struct ClusterIDs :IRustResource
        {
            public StringFFI id;
            public StringFFI leftID;
            public StringFFI rightID;

            public static (ClusterIDs, FFIError) Alloc(string data)
            {
                var result = NativeMethods.CreateClusterIDsMustFree(data, out var resource);
                return (resource, result);
            }

            public void Free()
            {
                Clam.FFI.NativeMethods.DeleteClusterIDs(ref this);
            }
        }

    }





}
