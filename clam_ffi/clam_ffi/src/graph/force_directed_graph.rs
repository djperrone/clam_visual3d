use rand::seq::IteratorRandom;

use super::physics_node::PhysicsNode;
use super::spring::Spring;
use crate::ffi_impl::cluster_data_wrapper::ClusterDataWrapper;
use crate::utils::error::FFIError;
use crate::utils::types::{Graphf32, Treef32};
use crate::{debug, utils, CBFnNodeVisitor, CBFnNodeVisitorMut};
use std::collections::HashMap;

use std::sync::{Condvar, Mutex};

pub struct Status {
    pub data_ready: bool,
    pub force_shutdown: bool,
}

impl Status {
    pub fn new() -> Self {
        Status {
            // this prevents thread from beginning work immediately - true
            data_ready: true,
            force_shutdown: false,
        }
    }
}

pub struct ForceDirectedGraph {
    graph: Mutex<(Status, HashMap<String, PhysicsNode>)>,
    edges: Vec<Spring>,
    pub max_edge_len: f32,
    pub scalar: f32,
    cond_var: Condvar,
    max_iters: i32,
}

impl ForceDirectedGraph {
    pub fn new(
        graph: HashMap<String, PhysicsNode>,
        edges: Vec<Spring>,
        scalar: f32,
        max_iters: i32,
    ) -> Self {
        let max_edge_len = Self::calc_max_edge_len(&edges);

        ForceDirectedGraph {
            graph: Mutex::new((Status::new(), graph)),
            edges,
            max_edge_len,
            scalar,
            cond_var: Condvar::new(),
            max_iters,
        }
    }

    fn compute_next_frame(&self) -> bool {
        let mutex_result = self
            .cond_var
            .wait_while(self.graph.lock().unwrap(), |(status, _)| {
                status.data_ready && !status.force_shutdown
            });

        match mutex_result {
            Ok(mut g) => {
                if g.0.force_shutdown {
                    g.0.data_ready = false;
                    return false;
                } else {
                    // if self.edges.is_empty() {
                    //     debug!("no edges in produce comp");
                    //     g.0.data_ready = false;
                    //     return false;
                    // }
                    for spring in self.edges.iter() {
                        spring.move_nodes(&mut g.1, self.max_edge_len, self.scalar);
                    }

                    g.0.data_ready = true;
                }
            }
            Err(e) => {
                debug!("graph mutex error? {}", e);
            }
        }

        true
    }

    unsafe fn force_shutdown(&self) -> FFIError {
        debug!("trying to end sim early - force shutdown lock");

        match self.graph.lock() {
            Ok(mut g) => {
                g.0.force_shutdown = true;
                self.cond_var.notify_all();
                FFIError::PhysicsRunning
            }
            Err(e) => {
                debug!("Mutex poisoned with error: {}", e);
                FFIError::PhysicsNotReady
            }
        }
    }

    unsafe fn try_update_unity(
        &self,
        clam_graph: &Graphf32,
        tree: &Treef32,
        updater: CBFnNodeVisitor,
    ) -> FFIError {
        match self.graph.try_lock() {
            Ok(mut g) => {
                let mut rng = rand::thread_rng();
                for cluster1 in clam_graph.clusters() {
                    for _ in 0..3 {
                        if let Some(cluster2) = clam_graph.clusters().iter().choose(&mut rng) {
                            let dist = cluster1.distance_to_other(tree.data(), cluster2);

                            let spring = Spring::new(dist, cluster1.name(), cluster2.name(), false);

                            spring.move_nodes(&mut g.1, self.max_edge_len, self.scalar);
                        }
                    }
                }
                for (key, value) in &mut g.1 {
                    value.update_position();
                    let baton_data =
                        ClusterDataWrapper::from_physics(key.as_str(), value.get_position());

                    updater(Some(baton_data.data()));
                }

                g.0.data_ready = false;
                self.cond_var.notify_one();

                FFIError::PhysicsRunning
            }
            Err(_) => {
                // debug!("Data not ready...try again later {}", e);
                FFIError::PhysicsNotReady
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

pub fn produce_computations(force_directed_graph: &ForceDirectedGraph) {
    for _ in 0..force_directed_graph.max_iters {
        // returns false if being forced to terminate mid - simulation
        if !force_directed_graph.compute_next_frame() {
            return;
        };
    }
}

pub unsafe fn try_update_unity(
    force_directed_graph: &ForceDirectedGraph,
    clam_graph: &Graphf32,
    tree: &Treef32,
    updater: CBFnNodeVisitor,
) -> FFIError {
    force_directed_graph.try_update_unity(clam_graph, tree, updater)
}

pub unsafe fn force_shutdown(force_directed_graph: &ForceDirectedGraph) -> FFIError {
    force_directed_graph.force_shutdown()
}



pub fn init_unity_edges(force_directed_graph: &ForceDirectedGraph, init_edges: CBFnNodeVisitorMut) {
    for edge in &force_directed_graph.edges {
        let mut data_wrapper = ClusterDataWrapper::default();
        let (id1, id2) = edge.get_node_ids();
        data_wrapper.data_mut().set_id(id1.clone());
        let mut msg = (edge.is_detected as i32).to_string();
        msg.push(' ');
        msg.push_str(id2.clone().as_str());
        data_wrapper.data_mut().set_message(msg);

        init_edges(Some(&mut data_wrapper.data_mut()));
    }
}
