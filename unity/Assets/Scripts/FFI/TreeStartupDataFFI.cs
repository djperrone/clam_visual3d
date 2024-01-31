
using System.Diagnostics;
using System.Runtime.InteropServices;
using Unity.Collections;

namespace Clam
{
    namespace FFI
    {
        [StructLayout(LayoutKind.Sequential)]
        public struct TreeStartupDataFFI : IRustResource
        {
            public StringFFI dataName;
            public DistanceMetric distanceMetric;
            public uint cardinality;
            public bool isExpensive;

            public static (TreeStartupDataFFI, FFIError) Alloc(TreeStartupData data)
            {
                (var resource, var result) = StringFFI.Alloc(data.dataName);
                
                TreeStartupDataFFI outData = new TreeStartupDataFFI();
                outData.dataName = resource;
                outData.distanceMetric = data.distanceMetric;
                outData.cardinality = data.cardinality;
                outData.isExpensive = data.isExpensive;

                return (outData, result);
            }

            public void Free()
            {
                NativeMethods.FreeString(ref dataName);
            }
        }
    }
}