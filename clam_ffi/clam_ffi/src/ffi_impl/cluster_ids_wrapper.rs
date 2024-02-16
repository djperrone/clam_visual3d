use crate::ffi_impl::cluster_ids::ClusterIDs;
use crate::utils::types::Vertexf32;

pub struct ClusterIDsWrapper {
    data: ClusterIDs,
}
impl Drop for ClusterIDsWrapper {
    fn drop(&mut self) {
        self.data.free_ids();
    }
}

impl ClusterIDsWrapper {
    pub fn from_cluster(cluster: &Vertexf32) -> Self {
        ClusterIDsWrapper {
            data: ClusterIDs::from_clam(cluster),
        }
    }

    pub fn data(&self) -> &ClusterIDs {
        &self.data
    }
    // pub fn data_mut(&mut self) -> &mut ClusterIDs {
    //     &mut self.data
    // }
}
