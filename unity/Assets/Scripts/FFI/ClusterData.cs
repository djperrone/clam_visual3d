using Clam.FFI;
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
        [Serializable]
        [StructLayout(LayoutKind.Sequential)]
        public partial struct ClusterData : IRustResource
        {
            public int depth;
            public int offset;
            public int cardinality;
            public int argCenter;
            public int argRadial;
            public float radius;
            public float lfd;

            public int vertexDegree;
            public float distToQuery;

            public Vec3 pos;
            public Vec3 color;
            public StringFFI id;
            public StringFFI message;

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
                Debug.Log("depth: " + this.depth);
                Debug.Log("cardinality: " + this.cardinality);
                Debug.Log("argCenter: " + this.argCenter);
                Debug.Log("argRadius: " + this.argRadial);
            }

            public string GetInfo()
            {
                StringBuilder stringBuilder = new StringBuilder();
                stringBuilder.AppendLine("id: " + this.id.AsString);
                stringBuilder.AppendLine("depth " + depth.ToString());
                stringBuilder.AppendLine("card: " + cardinality.ToString());
                stringBuilder.AppendLine("radius: " + radius.ToString());
                stringBuilder.AppendLine("lfd: " + lfd.ToString());
                stringBuilder.AppendLine("argC: " + argCenter.ToString());
                stringBuilder.AppendLine("argR: " + argRadial.ToString());
                return stringBuilder.ToString();
            }
            public string GetInfoForUI()
            {
                StringBuilder stringBuilder = new StringBuilder();

                stringBuilder.AppendLine(this.id.AsString);
                stringBuilder.AppendLine(depth.ToString());
                stringBuilder.AppendLine(cardinality.ToString());
                stringBuilder.AppendLine(offset.ToString());
                stringBuilder.AppendLine(radius.ToString());
                stringBuilder.AppendLine(lfd.ToString());
                stringBuilder.AppendLine(argCenter.ToString());
                stringBuilder.AppendLine(argRadial.ToString());
                return stringBuilder.ToString();
            }
            public void Free()
            {
                Clam.FFI.NativeMethods.DeleteClusterData(ref this);
            }
        }

        [Serializable]
        [StructLayout(LayoutKind.Sequential)]
        public partial struct ClusterIDs : IRustResource
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
