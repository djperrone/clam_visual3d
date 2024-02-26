use rand::seq::IteratorRandom;

use super::force_directed_graph::ForceDirectedGraph;
use super::physics_node::PhysicsNode;
use super::spring::Spring;
use crate::ffi_impl::cluster_data_wrapper::ClusterDataWrapper;
use crate::utils::error::FFIError;
use crate::utils::types::{Graphf32, Treef32};
use crate::{debug, utils, CBFnNodeVisitor, CBFnNodeVisitorMut};
use std::collections::HashMap;

use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};

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

pub struct ForceDirectedGraphAsync {
    graph: Mutex<(Status, ForceDirectedGraph)>,
    cond_var: Condvar,
}

impl ForceDirectedGraphAsync {
    pub fn new(graph: ForceDirectedGraph) -> Self {
        // let force_directed_graph = Arc::new(graph);

        // let b = force_directed_graph.clone();
        // let p = thread::spawn(move || {
        //     produce_computations(&b);
        // });
        // return Ok((p, force_directed_graph.clone()));
        Self {
            graph: Mutex::new((Status::new(), graph)),
            cond_var: Condvar::new(),
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
                    g.1.accumulate_forces();
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

    fn try_update_unity(
        &self,
        clam_graph: &Graphf32,
        tree: &Treef32,
        updater: Option<CBFnNodeVisitor>,
    ) -> FFIError {
        match self.graph.try_lock() {
            Ok(mut g) => {
                // let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
                // for cluster1 in clam_graph.clusters() {
                //     for _ in 0..3 {
                //         if let Some(cluster2) = clam_graph.clusters().iter().choose(&mut rng) {
                //             let dist = cluster1.distance_to_other(tree.data(), cluster2);

                //             let spring = Spring::new(dist, cluster1.name(), cluster2.name(), false);

                //             spring.move_nodes(&mut g.1, self.max_edge_len, self.scalar);
                //         }
                //     }
                // }

                g.1.apply_forces(clam_graph, tree, updater);

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
}

pub fn produce_computations(force_directed_graph: &ForceDirectedGraphAsync) {
    for _ in 0..force_directed_graph.max_iters {
        // returns false if being forced to terminate mid - simulation
        if !force_directed_graph.compute_next_frame() {
            return;
        };
    }
}

pub fn try_update_unity(
    force_directed_graph: &ForceDirectedGraphAsync,
    clam_graph: &Graphf32,
    tree: &Treef32,
    updater: Option<CBFnNodeVisitor>,
) -> FFIError {
    force_directed_graph.try_update_unity(clam_graph, tree, updater)
}

pub unsafe fn force_shutdown(force_directed_graph: &ForceDirectedGraphAsync) -> FFIError {
    force_directed_graph.force_shutdown()
}

pub fn init_unity_edges(
    force_directed_graph: &ForceDirectedGraphAsync,
    init_edges: CBFnNodeVisitorMut,
) {
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
