use std::collections::HashMap;

use super::h_node::PhysicsNode;

use crate::{ffi_impl::cluster_ids::ClusterID, graph};

#[derive(Debug)]
pub struct Spring {
    nat_len: f32,
    k_attractive: f32,
    k_repulsive: f32,
    node1: ClusterID, //String's reference hash table
    node2: ClusterID,
    relation: u32,
}

impl Spring {
    pub fn new(
        nat_len: f32,
        hash_code1: ClusterID,
        hash_code2: ClusterID,
        relation: u32,
        // normalize_len: Option<f32>,
        // scalar: Option<f32>,
    ) -> Self {
        Spring {
            nat_len,
            // nat_len: (nat_len / normalize_len.unwrap_or(1.0f32).max(f32::MIN))
            //     * scalar.unwrap_or(1.0f32), //: nat_len.min(1.0),
            k_attractive: 0.005,
            k_repulsive: 0.005,
            // k_repulsive: 0.002,
            node1: hash_code1,
            node2: hash_code2,
            relation,
        }
    }

    pub fn relation(&self) -> u32 {
        self.relation
    }

    pub fn get_node_ids(&self) -> (&ClusterID, &ClusterID) {
        (&self.node1, &self.node2)
    }

    //apply acceleration to both nodes at each end of spring
    pub fn move_nodes(&self, nodes: &mut HashMap<(usize, usize), PhysicsNode>, longest_edge: f32,
        scalar: f32, temp: Option<f32>) {
        //borrow ownership of nodes spring is connected to
        let node1 = nodes.get(&self.node1.to_tuple()).unwrap();
        let node2 = nodes.get(&self.node2.to_tuple()).unwrap();
        let force = node2.get_position() - node1.get_position();
        let force_magnitude = force.length();
        let target_len = (self.nat_len / longest_edge.max(f32::MIN)) * scalar;
        let new_magnitude = self.k_attractive * (force_magnitude - (target_len));
        let new_magnitude =
            new_magnitude * temp.unwrap_or(1.0f32);

        let mut new_force = graph::helpers::set_magnitude(force, new_magnitude);

        let node1 = nodes.get_mut(&self.node1.to_tuple()).unwrap();
        node1.accelerate(new_force);
        //reverse direction of force for node on opposite side
        new_force *= -1.;

        let node2 = nodes.get_mut(&self.node2.to_tuple()).unwrap();
        node2.accelerate(new_force);
    }

    pub fn nat_len(&self) -> f32 {
        self.nat_len
    }
}
