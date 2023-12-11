use std::collections::HashMap;

use super::physics_node::PhysicsNode;

use crate::graph;
#[derive(Debug)]
pub struct Spring {
    nat_len: f32,
    k: f32,
    node1: String, //String's reference hash table
    node2: String,
    pub is_detected: bool,
}

impl Spring {
    pub fn new(nat_len: f32, hash_code1: String, hash_code2: String, real: bool) -> Self {
        Spring {
            nat_len,
            k: 0.005,
            node1: hash_code1,
            node2: hash_code2,
            is_detected: real,
        }
    }

    pub fn get_node_ids(&self) -> (&String, &String) {
        (&self.node1, &self.node2)
    }

    //apply acceleration to both nodes at each end of spring
    pub fn move_nodes(
        &self,
        nodes: &mut HashMap<String, PhysicsNode>,
        longest_edge: f32,
        scalar: f32,
    ) {
        //borrow ownership of nodes spring is connected to
        let node1 = nodes.get(&self.node1).unwrap();
        let node2 = nodes.get(&self.node2).unwrap();

        let force = node2.get_position() - node1.get_position();
        let force_magnitude = force.length();
        let target_len = (self.nat_len / longest_edge.max(f32::MIN)) * scalar;
        let new_magnitude = self.k * (force_magnitude - (target_len));

        let mut new_force = graph::helpers::set_magnitude(force, new_magnitude);

        let node1 = nodes.get_mut(&self.node1).unwrap();
        node1.accelerate(new_force);
        //reverse direction of force for node on opposite side
        new_force *= -1.;

        let node2 = nodes.get_mut(&self.node2).unwrap();
        node2.accelerate(new_force);
    }

    pub fn nat_len(&self) -> f32 {
        self.nat_len
    }
}
