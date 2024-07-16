use std::collections::HashMap;

use super::physics_node::PhysicsNode;

use crate::{ffi_impl::cluster_ids::ClusterID, graph};
#[derive(Debug)]
pub struct Spring {
    nat_len: f32,
    k: f32,
    node1: ClusterID, //String's reference hash table
    node2: ClusterID,
    pub is_real: bool,
}

impl Spring {
    pub fn new(nat_len: f32, hash_code1: ClusterID, hash_code2: ClusterID, real: bool) -> Self {
        Spring {
            nat_len, //: nat_len.min(1.0),
            k: 0.005,
            node1: hash_code1,
            node2: hash_code2,
            is_real: real,
        }
    }

    pub fn get_node_ids(&self) -> (&ClusterID, &ClusterID) {
        (&self.node1, &self.node2)
    }

    //apply acceleration to both nodes at each end of spring
    pub fn move_nodes(
        &self,
        nodes: &mut HashMap<(usize, usize), PhysicsNode>,
        longest_edge: f32,
        scalar: f32,
    ) {
        //borrow ownership of nodes spring is connected to
        let node1 = nodes.get(&self.node1.to_tuple()).unwrap();
        let node2 = nodes.get(&self.node2.to_tuple()).unwrap();
        let force = node2.get_position() - node1.get_position();
        let force_magnitude = force.length();
        // let min_length = 20.;
        let target_len = (self.nat_len / longest_edge.max(f32::MIN)) * scalar;
        // let target_len = (self.nat_len / longest_edge.max(f32::MIN)).max(min_length) * scalar;
        // let target_len = ((self.nat_len / longest_edge.max(f32::MIN)) * scalar).max(min_length);
        let new_magnitude = self.k * (force_magnitude - (target_len));

        // Scale the force magnitude if the spring is not real
        // let scaled_magnitude = if !self.is_real {
        //     new_magnitude / 2.0 // Adjust the scaling factor as needed
        // } else {
        //     new_magnitude
        // };

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
