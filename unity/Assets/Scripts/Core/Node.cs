using UnityEngine;

//public interface IClickable
//{
//    void OnClick();
//}
namespace Clam
{


    public class Node : MonoBehaviour
    {
        private static Color m_SelectedColor = new Color(0.0f/255.0f, 231.0f/ 255.0f, 250.0f / 255.0f);
        private string m_ID;
        private string m_LeftChildID;
        private string m_RightChildID;

        //public int test = 5;
        private bool m_IsSelected = false;
        private Color m_ActualColor;
        //private InputAction click;
        public float m_DistanceToQuery = -1.0f;

        public bool Selected
        {
            get { return m_IsSelected; }
            set { m_IsSelected = value; }
        }

        private int m_IndexBufferID;

        public int IndexBufferID
        {
            get { return m_IndexBufferID; }
            set { m_IndexBufferID = value; }
        }

        // Start is called before the first frame update
        void Start()
        {
            m_IndexBufferID = -1;
        }
        void Update()
        {
        }

        public void ToggleSelect()
        {
            if (m_IsSelected)
            {
                Deselect();
            }
            else
            {
                Select();
            }
        }

        public void Select()
        {
            GetComponent<Renderer>().material.color = new Color(0.0f, 0.0f, 1.0f);
            m_IsSelected = true;
        }

        public void Deselect()
        {
            SetColor(m_ActualColor);
            m_IsSelected = false;
        }

        public void ExpandSubtree()
        {
            //ClamFFI.Clam.ForEachDFT(m_ExpandSubtree, this._id);
        }


        public void SetPosition(Vector3 pos)
        {
            GetComponent<Transform>().position = new Vector3(pos.x, -pos.y, pos.z);
        }

        public void SetColor(Color color)
        {
            GetComponent<Renderer>().material.color = color;
            m_ActualColor = color;
        }

        public bool IsSelected()
        {
            return m_IsSelected;
        }

        public string GetId()
        {
            return m_ID;
        }

        public string GetLeftChildID()
        {
            return m_LeftChildID;
        }
        public string GetRightChildID()
        {
            return m_RightChildID;
        }
        public Vector3 GetPosition()
        {
            return GetComponent<Transform>().position;
        }

        public Color GetColor()
        {
            return GetComponent<Renderer>().material.color;
        }
        public Color GetActualColor()
        {
            return m_ActualColor;
        }

        public void SetActualColor(Color color)
        {
            m_ActualColor = color;
            SetColor(color);
        }

        public void SetID(string id)
        {
            m_ID = id;
        }

        public void SetLeft(string id)
        {
            m_LeftChildID = id;
        }

        public void SetRight(string id)
        {
            m_RightChildID = id;
        }

        public bool IsLeaf()
        {
            //print("-------------------------------------------------lid " + m_LeftChildID + ", rid " + m_RightChildID);
            return m_LeftChildID == "None" && m_RightChildID == "None";
        }
    }
}

