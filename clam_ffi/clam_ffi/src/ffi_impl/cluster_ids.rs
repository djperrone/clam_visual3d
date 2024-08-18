use abd_clam::Cluster;

use crate::utils::types::Vertexf32;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ClusterIDs {
    pub id: ClusterID,
    pub left_id: ClusterID,
    pub right_id: ClusterID,


}

impl ClusterIDs {
    pub unsafe fn get_id(&self) -> ClusterID {
        self.id
    }

    // pub unsafe fn get_ffi_id(&self) -> &StringFFI {
    //     &self.id
    // }

    pub fn new(id : ClusterID, left_id : ClusterID, right_id : ClusterID)->Self{
        ClusterIDs {
            id,
            left_id,
            right_id,
        }
    }

    pub fn from_clam(node: &Vertexf32) -> Self {
        let (left_id, right_id) = {
            if let Some([left, right]) = node.children() {
                (ClusterID{offset : left.offset(), cardinality : left.cardinality()}, ClusterID{offset : right.offset(), cardinality : right.cardinality()})
            } else {
                (ClusterID::new(0,0), ClusterID::new(0,0))
            }
        };

        ClusterIDs {
            id: ClusterID::new(node.offset(), node.cardinality()),
            left_id,
            right_id,
        }
    }

    // pub fn free_ids(&mut self) {
    //     self.id.free_data();
    //     self.left_id.free_data();
    //     self.right_id.free_data();
    // }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ClusterID {
   offset : usize,
   cardinality: usize,
}

impl ClusterID{
    pub fn new(offset : usize, cardinality : usize)->Self{
        ClusterID{
            offset : offset,
            cardinality : cardinality
        }
    }

    pub fn from_cluster(cluster : &Vertexf32) -> Self{
        ClusterID{
            offset : cluster.offset(),
            cardinality : cluster.cardinality()
        }
    }

    pub fn to_tuple(&self)->(usize, usize){
        (self.offset, self.cardinality)
    }
}
