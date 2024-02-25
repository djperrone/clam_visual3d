use super::cluster_data::ClusterData;
use crate::{tree_layout::reingold_impl, utils::types::Clusterf32};
pub struct ClusterDataWrapper {
    data: ClusterData,
}

impl Drop for ClusterDataWrapper {
    fn drop(&mut self) {
        self.data.free_ids();
    }
}

impl ClusterDataWrapper {
    pub fn default() -> Self {
        ClusterDataWrapper {
            data: ClusterData::default(),
        }
    }
    pub fn from_cluster(cluster: &Clusterf32) -> Self {
        ClusterDataWrapper {
            data: ClusterData::from_clam(cluster),
        }
    }
    pub fn from_physics(id: &str, pos: glam::Vec3) -> Self {
        ClusterDataWrapper {
            data: ClusterData::from_physics(id, pos),
        }
    }
    pub fn from_reingold_node(node: &reingold_impl::Node) -> Self {
        ClusterDataWrapper {
            data: ClusterData::from_reingold_node(node),
        }
    }

    pub fn data(&self) -> &ClusterData {
        &self.data
    }
    pub fn data_mut(&mut self) -> &mut ClusterData {
        &mut self.data
    }
}
