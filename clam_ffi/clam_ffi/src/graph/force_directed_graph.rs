use abd_clam::Cluster;
use rand::seq::IteratorRandom;

use super::physics_node::PhysicsNode;
use crate::h_graph::h_spring::Spring;
use crate::ffi_impl::cluster_data::ClusterData;
use crate::ffi_impl::cluster_ids::{ClusterID, ClusterIDs};
use crate::h_graph::h_graph;
// use crate::ffi_impl::cluster_data_wrapper::ClusterDataWrapper;
use crate::utils::error::FFIError;
use crate::utils::types::{Graphf32, Treef32};
use crate::{debug, CBFnNameSetter, CBFnNodeVisitor};
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

pub struct ForceDirectedGraphAsync {
    graph: Mutex<(Status, h_graph::ForceDirectedGraph)>,
    // edges: Vec<Spring>,
    pub max_edge_len: f32,
    pub scalar: f32,
    cond_var: Condvar,
    max_iters: i32,
    max_depth : usize,
}

impl ForceDirectedGraphAsync {
    pub fn new(
        graph: h_graph::ForceDirectedGraph,
        // edges: Vec<Spring>,
        scalar: f32,
        max_iters: i32,
        max_depth : usize,
    ) -> Self {
        let max_edge_len = Self::calc_max_edge_len(&graph.springs());
        
        ForceDirectedGraphAsync {
            graph: Mutex::new((Status::new(), graph)),
            // edges,
            max_depth,
            max_edge_len,
            scalar,
            cond_var: Condvar::new(),
            max_iters,
        }
    }

    pub fn update(&mut self, clam_graph: &Graphf32, tree: &Treef32) {
        match self.graph.get_mut() {
            Ok(g) => {
                // for spring in g.1.springs().iter() {
                //     spring.move_nodes(g.1.graph_mut(), self.max_edge_len, self.scalar, None);
                // }
                g.1.move_nodes();
                
                Self::apply_forces(g.1.graph_mut());
            }
            Err(_) => {}
        }
    }

    pub fn graph_mut(&mut self) -> Result<&mut HashMap<(usize, usize), PhysicsNode>, String> {
        match self.graph.get_mut() {
            Ok(graph) => Ok(graph.1.graph_mut()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn get_cluster_position(&self, id: (usize, usize)) -> Result<glam::Vec3, String> {
        match self.graph.try_lock() {
            Ok(g) => {
                if let Some(c) = g.1.graph().get(&id) {
                    return Ok(c.get_position());
                } else {
                    return Err("Cluster not found".to_string());
                }
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }

    // pub fn edges_mut(&mut self) -> &mut Vec<Spring> {
    //     &mut self.edges
    // }

    // pub fn add_edge(&mut self, edge: Spring) {
    //     self.edges.push(edge);
    // }

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
                }else if g.1.cur_depth == g.1.max_depth{
                    return false;
                } else {
                    // g.1.move_nodes();
                    g.1.accumulate_forces();
                    g.1.cur_iter+=1;
                    // Self::accumulate_edge_forces(
                    //     g.1,
                    //     // g.1.springs(),
                    //     // self.max_edge_len,
                    //     // self.scalar,
                    // );
                    g.0.data_ready = true;
                }
            }
            Err(e) => {
                debug!("graph mutex error? {}", e);
            }
        }
        true
    }

    // pub fn accumulate_edge_forces(
    //     graph: &mut h_graph::ForceDirectedGraph,
    //     // edges: &Vec<Spring>,
    //     // max_edge_len: f32,
    //     // scalar: f32,
    // ) {
    //     // for spring in edges.iter() {
    //     //     spring.move_nodes(graph, max_edge_len, scalar, None);
    //     // }
    //     graph.move_nodes();
    // }

    // pub fn accumulate_random_forces(
    //     graph: &mut HashMap<(usize, usize), PhysicsNode>,
    //     clam_graph: &Graphf32,
    //     tree: &Treef32,
    //     max_edge_len: f32,
    //     scalar: f32,
    // ) {
    //     let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    //     for cluster1 in clam_graph.ordered_clusters() {
    //         for _ in 0..3 {
    //             if let Some(cluster2) = clam_graph.ordered_clusters().iter().choose(&mut rng) {
    //                 let dist = cluster1.distance_to_other(tree.data(), cluster2);

    //                 let spring = Spring::new(dist, ClusterID::from_cluster(&cluster1), ClusterID::from_cluster(&cluster2), false);

    //                 spring.move_nodes(graph, max_edge_len, scalar);
    //             }
    //         }
    //     }
    // }

    pub fn apply_forces(graph: &mut HashMap<(usize, usize), PhysicsNode>) {
        for (_, value) in graph {
            value.update_position();
        }
    }

    pub fn apply_forces_and_update_unity(
        graph: &mut HashMap<(usize, usize), PhysicsNode>,
        updater: CBFnNodeVisitor,
    ) {
        for (key, value) in graph {
            value.update_position();
            let data = ClusterData::from_physics(*key, value.get_position());

            updater(Some(&data));
        }
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

    // Need to pass callback to update node visinbility and rebuilkd edge mesh
    unsafe fn try_update_unity(
        &self,
        clam_graph: &Graphf32,
        tree: &Treef32,
        reset_graph_dict_cb: CBFnNodeVisitor,
        refill_graph_dict_cb: CBFnNodeVisitor,
        reset_mesh_cb: CBFnNodeVisitor,
        rebuild_edges_cb: CBFnNameSetter,
        update_pos_cb: CBFnNodeVisitor,
    ) -> FFIError {
        match self.graph.try_lock() {
            Ok(mut g) => {
                // Self::accumulate_random_forces(
                //     &mut g.1,
                //     clam_graph,
                //     tree,
                //     self.max_edge_len,
                //     self.scalar,
                // );
                // if g.1.cur_depth == 1 && g.1.cur_iter == 0{
                //     debug!("RESETTING UNITY GRAPH 1st");
                   
                //     reset_graph_dict_cb(Some(&ClusterData::default()));
                //     for cluster in g.1.graph(){
                //         refill_graph_dict_cb(Some(&ClusterData::from_physics(*cluster.0, cluster.1.get_position())))
                //     }
                //     let mut cluster_data = ClusterData::default();
                //     cluster_data.depth = g.1.springs().len() as i32;
                //     reset_mesh_cb(Some(&cluster_data));
                //     init_unity_edges(self, rebuild_edges_cb)
                //     // rebuild_edges_cb(Some(&ClusterData::default()));
                // }


                if g.1.cur_iter == g.1.max_iters{
                    if g.1.cur_depth == g.1.max_depth{
                         // if g.1.cur_depth >= g.1.max_depth{
                        g.1.cleanup( tree, None);
                        g.0.force_shutdown = true;
                        
                        // return FFIError::Ok;
                    // }
                    }else{
                        g.1.cur_iter = 0;
                        g.1.split(tree, clam_graph);
                        g.1.cur_depth+=1;
                    }
                    debug!("RESETTING UNITY GRAPH 2nc");

                    reset_graph_dict_cb(Some(&ClusterData::default()));
                    for cluster in g.1.graph(){
                        refill_graph_dict_cb(Some(&ClusterData::from_physics(*cluster.0, cluster.1.get_position())))
                    }
                    let mut cluster_data = ClusterData::default();
                    cluster_data.depth = g.1.springs().len() as i32;
                    reset_mesh_cb(Some(&cluster_data));
                    for edge in g.1.springs() {
                        let (id1, id2) = edge.get_node_ids();
                        let mut data = ClusterIDs::new(*id1, *id2, ClusterID::new(true as usize, 0));
                        rebuild_edges_cb(Some(&mut data));
                }
                    // init_unity_edges(self, rebuild_edges_cb)
                    // rebuild_edges_cb(Some(&ClusterData::default()));
                    // need to reset unity graph?
                    // node visinility callback
                    // edge reset callback
                }
                if g.1.cur_depth <= g.1.max_depth{
                    Self::apply_forces_and_update_unity(g.1.graph_mut(), update_pos_cb);
                    g.1.cleanup( tree, None);

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

pub fn produce_computations(force_directed_graph: &ForceDirectedGraphAsync) {

    // for cur_depth in 0..force_directed_graph.max_depth {
    //     for i in 0..force_directed_graph.max_iters {
    //     // returns false if being forced to terminate mid - simulation
    //     if !force_directed_graph.compute_next_frame() {
    //         return;
    //     };
    //     }
    // }
        while force_directed_graph.compute_next_frame() {}

}

pub unsafe fn try_update_unity(
    force_directed_graph: &ForceDirectedGraphAsync,
    clam_graph: &Graphf32,
    tree: &Treef32,
    reset_graph_dict_cb: CBFnNodeVisitor,
    refill_graph_dict_cb: CBFnNodeVisitor,
    reset_mesh_cb: CBFnNodeVisitor,
    rebuild_edges_cb: CBFnNameSetter,
    update_pos_cb: CBFnNodeVisitor,
) -> FFIError {
    force_directed_graph.try_update_unity(clam_graph, tree, reset_graph_dict_cb, refill_graph_dict_cb,reset_mesh_cb,rebuild_edges_cb, update_pos_cb)
}

pub unsafe fn force_shutdown(force_directed_graph: &ForceDirectedGraphAsync) -> FFIError {
    force_directed_graph.force_shutdown()
}

// The offset of the right id represents i f this edge is real or not
pub fn init_unity_edges(force_directed_graph: &ForceDirectedGraphAsync, init_edges: CBFnNameSetter) {
    match force_directed_graph.graph.try_lock() {
        Ok(g) => {
            for edge in g.1.springs() {
            let (id1, id2) = edge.get_node_ids();
            let mut data = ClusterIDs::new(*id1, *id2, ClusterID::new(true as usize, 0));
            init_edges(Some(&mut data));
    }
        }
        Err(e) => {
            
        }
    }
   
}
