//using Clam;
//using Clam.FFI;
//using System.Collections;
//using System.Collections.Generic;
//using System.Linq;
//using UnityEngine;

//public class GraphComponent : MonoBehaviour
//{
//    List<string> m_ClusterIDs;
//    List<Edge> m_Edges;
//    Rigidbody m_RigidBody;
//    int m_ID;


//    // Start is called before the first frame update
//    void Start()
//    {

//    }

//    void Init(int componentID, List<string> clusterIDs)
//    {
//        m_ID = componentID;
//        m_ClusterIDs = clusterIDs;
//        foreach (var id in m_ClusterIDs)
//        {
//            if (Cakes.Tree.GetTree().TryGetValue(id, out var cluster))
//            {
//                cluster.gameObject.transform.SetParent(gameObject.transform);
//            }
//        }
//    }

//    // Update is called once per frame
//    void Update()
//    {

//    }

//    public List<string> GetRepresentatives()
//    {
//        List<string> chosen = new List<string>();

//        for (int i = 0; i < 3 && i < m_ClusterIDs.Count; i++)
//        {
//            chosen.Add(m_ClusterIDs[i]);
//        }
//        return chosen;

//    }
//}
