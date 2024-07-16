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
        public partial struct ClusterData //: IRustResource
        {
            public int depth;
            public nuint offset;
            public nuint cardinality;
            public int argCenter;
            public int argRadial;
            public float radius;
            public float lfd;
            public int vertexDegree;
            public float distToQuery;

            public Vec3 pos;
            public Vec3 color;
            //public StringFFI id;
            //public StringFFI message;

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

            //public static (ClusterData, FFIError) Alloc((int, int) data)
            //{
            //    var result = NativeMethods.CreateClusterDataMustFree(data, out var resource);
            //    return (resource, result);
            //}

            public ClusterID ID()
            {
                return new ClusterID (offset, cardinality);
            }

            public string ID_AsString()
            {
                return (offset, cardinality).ToString();
            }

            public (nuint, nuint) ID_AsTuple()
            {
                return (offset, cardinality);
            }

            public void LogInfo()
            {
                Debug.Log("id: " + this.ID().ToString());
                Debug.Log("pos: " + this.pos.AsVector3);
                Debug.Log("color: " + this.color.AsColor);
                Debug.Log("depth: " + this.depth);
                Debug.Log("offset: " + this.offset);
                Debug.Log("cardinality: " + this.cardinality);
                Debug.Log("argCenter: " + this.argCenter);
                Debug.Log("argRadius: " + this.argRadial);
            }

            public string GetInfo()
            {
                StringBuilder stringBuilder = new StringBuilder();
                stringBuilder.AppendLine("id: " + this.ID().ToString());
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

                stringBuilder.AppendLine(this.ID().ToString());
                stringBuilder.AppendLine(depth.ToString());
                stringBuilder.AppendLine(cardinality.ToString());
                stringBuilder.AppendLine(offset.ToString());
                stringBuilder.AppendLine(radius.ToString());
                stringBuilder.AppendLine(lfd.ToString());
                stringBuilder.AppendLine(argCenter.ToString());
                stringBuilder.AppendLine(argRadial.ToString());
                return stringBuilder.ToString();
            }
            //public void Free()
            //{
            //    Clam.FFI.NativeMethods.DeleteClusterData(ref this);
            //}
        }

        [Serializable]
        [StructLayout(LayoutKind.Sequential)]
        public partial struct ClusterIDs
        {
            public ClusterID id;
            public ClusterID leftID;
            public ClusterID rightID;

            //public static (ClusterIDs, FFIError) Alloc(string data)
            //{
            //    var result = NativeMethods.CreateClusterIDsMustFree(data, out var resource);
            //    return (resource, result);
            //}

            //public void Free()
            //{
            //    Clam.FFI.NativeMethods.DeleteClusterIDs(ref this);
            //}
        }

        [Serializable]
        [StructLayout(LayoutKind.Sequential)]
        public partial struct ClusterID
        {
            public ClusterID(nuint offset, nuint cardinality)
            {
                this.offset = offset;
                this.cardinality = cardinality;
            }

            nuint offset, cardinality;

            public nuint Offset
            {
                get { return offset; }
            }
            public nuint Cardinality
            {
                get { return cardinality; }
            }

            public (nuint, nuint) AsTuple()
            {
                return (offset, cardinality);
            }

            public string AsString()
            {
                return AsTuple().ToString();
            }
        }


    }
}
