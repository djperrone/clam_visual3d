
using System;
using System.Diagnostics;
using System.Runtime.InteropServices;

namespace Clam
{
    namespace FFI
    {
        public interface IRustResource
        {
            void Free();
            // must also implement but its impossible to enforce...
            //public static (T, FFIError) Alloc(Args&&... data)
        }

        public class RustResourceWrapper<T> where T : struct, IRustResource
        {
            T m_Data;
            public FFIError result;

            public T Data
            {
                get { return m_Data; }
            }

            public RustResourceWrapper(T data, FFIError result)
            {
                m_Data = data;
                this.result = result;
            }

            public RustResourceWrapper((T data, FFIError result) tuple)
            {
                m_Data = tuple.data;
                this.result = tuple.result;
            }

            public T GetData()
            {
                return m_Data;
            }

            ~RustResourceWrapper()
            {
                m_Data.Free();
            }
        }
    }
}