use std::collections::HashMap;

use abd_clam::Cluster;
use rand::seq::IteratorRandom;

use super::{
    physics_node::{self, PhysicsNode},
    spring::Spring,
};
use crate::utils::types::{Graphf32, Treef32};

pub struct ForceDirectedGraph {
    graph: HashMap<String, PhysicsNode>,
    edges: Vec<Spring>,
    pub max_edge_len: f32,
    pub scalar: f32,
    max_iters: i32,
}

impl ForceDirectedGraph {
    pub fn new(
        graph: HashMap<String, PhysicsNode>,
        edges: Vec<Spring>,
        scalar: f32,
        max_iters: i32,
        max_edge_len: f32,
    ) -> Self {
        // let max_edge_len = Self::calc_max_edge_len(&edges);

        ForceDirectedGraph {
            graph: graph,
            edges,
            max_edge_len,
            scalar,
            max_iters,
        }
    }

    pub fn update(&mut self, clam_graph: &Graphf32, tree: &Treef32) {
        for spring in self.edges.iter() {
            spring.move_nodes(&mut self.graph); //, self.max_edge_len, self.scalar);
        }

        self.accumulate_random_forces(clam_graph, tree);

        self.apply_forces();
    }

    pub fn apply_forces(&mut self) {
        for (_, value) in &mut self.graph {
            value.update_position();
        }
    }

    pub fn get_cluster_position(&self, id: &String) -> Result<glam::Vec3, String> {
        if let Some(c) = self.graph.get(id) {
            return Ok(c.get_position());
        } else {
            return Err("Cluster not found".to_string());
        }
    }

    pub fn accumulate_edge_forces(
        &mut self,
        // max_edge_len: f32,
        // scalar: f32,
    ) {
        for spring in self.edges.iter() {
            spring.move_nodes(&mut self.graph); //, max_edge_len, scalar);
        }
    }

    pub fn accumulate_random_forces(&mut self, clam_graph: &Graphf32, tree: &Treef32) {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        for cluster1 in clam_graph.ordered_clusters() {
            for _ in 0..3 {
                if let Some(cluster2) = clam_graph.ordered_clusters().iter().choose(&mut rng) {
                    let dist = (cluster1.distance_to_other(tree.data(), cluster2)
                        / self.max_edge_len)
                        * self.scalar;

                    let spring = Spring::new(dist, cluster1.name(), cluster2.name(), false);

                    spring.move_nodes(&mut self.graph); //, max_edge_len, scalar);
                }
            }
        }
    }
}
