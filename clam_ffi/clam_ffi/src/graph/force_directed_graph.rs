use rand::seq::{IteratorRandom, SliceRandom};
use rand::Rng;

use super::physics_node::PhysicsNode;
use super::spring::Spring;
use crate::ffi_impl::cluster_data_wrapper::ClusterDataWrapper;
use crate::utils::types::{Clusterf32, DataSetf32, Graphf32, Treef32};
use crate::{debug, utils, CBFnNodeVisitor, CBFnNodeVisitorMut};
use std::collections::{HashMap, HashSet};

use std::sync::{Condvar, Mutex};

pub struct ForceDirectedGraph {
    graph: HashMap<String, PhysicsNode>,
    edges: Vec<Spring>,
    pub max_edge_len: f32,
    pub scalar: f32,
    cond_var: Condvar,
    max_iters: i32,
}

impl ForceDirectedGraph {
    pub fn new(tree: &Treef32, clam_graph: &Graphf32, scalar: f32, max_iters: i32) -> Self {
        let mut graph: HashMap<String, PhysicsNode> = HashMap::new();
        let mut rng = rand::thread_rng();

        for c in clam_graph.clusters().iter() {
            let x: f32 = rng.gen_range(0.0..=100.0);
            let y: f32 = rng.gen_range(0.0..=100.0);
            let z: f32 = rng.gen_range(0.0..=100.0);
            graph.insert(c.name(), PhysicsNode::new(glam::Vec3::new(x, y, z), c));
        }
        let mut springs = Vec::new();
        for e in clam_graph.edges() {
            springs.push(Spring::new(
                e.distance(),
                e.left().name(),
                e.right().name(),
                true,
            ));
        }

        Self::create_intercomponent_edges(tree.data(), clam_graph, &mut springs, 3);

        ForceDirectedGraph {
            graph: graph,
            max_edge_len: Self::calc_max_edge_len(&springs),
            edges: springs,

            scalar,
            cond_var: Condvar::new(),
            max_iters,
        }
    }

    // pub fn new(
    //     graph: HashMap<String, PhysicsNode>,
    //     edges: Vec<Spring>,
    //     scalar: f32,
    //     max_iters: i32,
    // ) -> Self {
    //     let max_edge_len = Self::calc_max_edge_len(&edges);

    //     ForceDirectedGraph {
    //         graph: graph,
    //         edges,
    //         max_edge_len,
    //         scalar,
    //         cond_var: Condvar::new(),
    //         max_iters,
    //     }
    // }

    pub fn add_edge(&mut self, edge: Spring) {
        self.edges.push(edge);
    }

    pub fn accumulate_forces(&mut self) {
        for spring in self.edges.iter() {
            spring.move_nodes(&mut self.graph, self.max_edge_len, self.scalar);
        }
    }

    fn get_k_key_clusters<'a>(
        clam_graph: &'a Graphf32,
        component: &'a HashSet<&'a Clusterf32>,
        k: usize,
    ) -> Option<Vec<&'a Clusterf32>> {
        let mut key_clusters: Vec<&Clusterf32> = Vec::new();
        let key_cluster = component.iter().max_by(|x, y| {
            clam_graph
                .vertex_degree(x)
                .cmp(&clam_graph.vertex_degree(y))
        });

        if let Some(kc) = key_cluster {
            key_clusters.push(kc);

            let mut comp: Vec<&Clusterf32> = component.iter().map(|x| *x).collect();

            let mut rng = rand::thread_rng();
            let (shuffled, _) = comp.partial_shuffle(&mut rng, k + 1);

            for c in shuffled {
                if key_clusters.len() < k {
                    if c != kc {
                        key_clusters.push(c);
                    }
                } else {
                    return Some(key_clusters);
                }
            }
        }

        return None;
    }

    fn cross_pollinate_components<'a>(
        key_clusters1: &Vec<&'a Clusterf32>,
        key_clusters2: &Vec<&'a Clusterf32>,
        data: &DataSetf32,
        edges: &mut Vec<Spring>,
    ) {
        for c1 in key_clusters1.iter() {
            for c2 in key_clusters2.iter() {
                let spring =
                    Spring::new(c1.distance_to_other(data, c2), c1.name(), c2.name(), false);
                edges.push(spring);
            }
        }
    }

    fn create_intercomponent_edges(
        data: &DataSetf32,
        clam_graph: &Graphf32,
        edges: &mut Vec<Spring>,
        k: usize,
    ) {
        let component_clusters = clam_graph.find_component_clusters();

        for (i, component) in component_clusters.iter().enumerate() {
            if let Some(key_clusters) = Self::get_k_key_clusters(clam_graph, component, k) {
                for component2 in component_clusters.iter().skip(i + 1) {
                    if let Some(key_clusters2) = Self::get_k_key_clusters(clam_graph, component2, k)
                    {
                        Self::cross_pollinate_components(&key_clusters, &key_clusters2, data, edges)
                    }
                }
            }
        }
    }

    pub fn apply_forces(
        &mut self,
        clam_graph: &Graphf32,
        tree: &Treef32,
        updater: Option<CBFnNodeVisitor>,
    ) {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        for cluster1 in clam_graph.clusters() {
            for _ in 0..3 {
                if let Some(cluster2) = clam_graph.clusters().iter().choose(&mut rng) {
                    let dist = cluster1.distance_to_other(tree.data(), cluster2);

                    let spring = Spring::new(dist, cluster1.name(), cluster2.name(), false);

                    spring.move_nodes(&mut self.graph, self.max_edge_len, self.scalar);
                }
            }
        }

        for (key, value) in &mut self.graph {
            value.update_position();

            if let Some(updater) = updater {
                for (key, value) in &mut self.graph {
                    value.update_position();
                    let baton_data =
                        ClusterDataWrapper::from_physics(key.as_str(), value.get_position());

                    updater(Some(baton_data.data()));
                }
            }
        }
    }

    fn calc_max_edge_len(edges: &[Spring]) -> f32 {
        match edges.iter().reduce(|cur_max: &Spring, val: &Spring| {
            if cur_max.nat_len() > val.nat_len() {
                cur_max
            } else {
                val
            }
        }) {
            Some(spring) => spring.nat_len(),
            None => 1.0,
        }

        // max_edge_len
    }
}
