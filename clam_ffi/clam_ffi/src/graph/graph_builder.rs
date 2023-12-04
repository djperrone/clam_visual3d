use std::{
    collections::HashMap,
    sync::Arc,
    thread::{self, JoinHandle},
};

use crate::{
    debug,
    // debug,
    ffi_impl::cluster_data::ClusterData,
    graph,
    handle::handle::Handle,
    utils::{
        error::FFIError,
        types::{Clusterf32, DataSet},
    },
};

type Edge = (String, String, f32, bool);

use super::{force_directed_graph::ForceDirectedGraph, physics_node::PhysicsNode, spring::Spring};

pub unsafe fn build_force_directed_graph(
    cluster_data_arr: &[ClusterData],
    handle: &Handle,
    scalar: f32,
    max_iters: i32,
    // edge_detector_cb: CBFnNodeVisitorMut,
    // physics_update_cb: CBFnNodeVisitor,
) -> Result<(JoinHandle<()>, Arc<ForceDirectedGraph>), FFIError> {
    let springs: Vec<Spring> = {
        let mut clusters: Vec<&Clusterf32> = Vec::new();

        for c in cluster_data_arr.iter() {
            if let Ok(cluster) = handle.get_cluster_from_string(c.id.as_string().unwrap()) {
                clusters.push(cluster);
            }
        }
        create_springs(detect_edges(&clusters, &handle.data())) //, edge_detector_cb))
    };

    let graph = build_graph(handle, &cluster_data_arr);
    if graph.len() == 0 || springs.len() == 0 {
        return Err(FFIError::GraphBuildFailed);
    }

    let force_directed_graph = Arc::new(ForceDirectedGraph::new(
        graph, springs, scalar, max_iters,
        // physics_update_cb,
    ));

    let b = force_directed_graph.clone();
    let p = thread::spawn(move || {
        graph::force_directed_graph::produce_computations(&b);
    });
    return Ok((p, force_directed_graph.clone()));
}

pub unsafe fn build_graph(
    // clusters: &'a Vec<&'a Clusterf32>,
    handle: &Handle,
    cluster_data_arr: &[ClusterData],
) -> HashMap<String, PhysicsNode> {
    let mut graph: HashMap<String, PhysicsNode> = HashMap::new();

    for c in cluster_data_arr {
        graph.insert(
            c.id.as_string().unwrap(),
            PhysicsNode::new(
                &c,
                handle
                    .get_cluster_from_string(c.id.as_string().unwrap())
                    .unwrap(),
            ),
        );
    }

    return graph;
}
// pub unsafe fn build_graph(
//     // clusters: &'a Vec<&'a Clusterf32>,
//     cluster_data_arr: &[NodeData],
// ) -> HashMap<String, PhysicsNode> {
//     let mut graph: HashMap<String, PhysicsNode> = HashMap::new();

//     for c in cluster_data_arr {
//         graph.insert(
//             c.id.as_string().unwrap(),
//             PhysicsNode::new(&c, handle.find_node(c.id.as_string().unwrap()).unwrap()),
//         );
//     }

//     return graph;
// }
//adding comment

pub fn detect_edges(
    clusters: &Vec<&Clusterf32>,
    dataset: &Option<&DataSet>,
    // node_visitor: crate::CBFnNodeVisitorMut,
) -> Vec<Edge> {
    let mut edges: Vec<Edge> = Vec::new();
    if let Some(data) = *dataset {
        for i in 0..clusters.len() {
            for j in (i + 1)..clusters.len() {
                let distance = clusters[i].distance_to_other(data, clusters[j]);
                if distance <= clusters[i].radius() + clusters[j].radius() {
                    edges.push((clusters[i].name(), clusters[j].name(), distance, true));
                } else {
                    // edges.push((clusters[i].name(), clusters[j].name(), distance, false));

                    // edges.push((
                    //     clusters[i].name(),
                    //     clusters[j].name(),
                    //     distance,
                    //     distance <= clusters[i].radius + clusters[j].radius,
                    // ));
                }

                // let mut baton = ClusterDataWrapper::from_cluster(clusters[i]);
                // baton.data_mut().set_message(clusters[j].name());
                // node_visitor(Some(baton.data_mut()));

                // debug!(
                //     "message from unity {}",
                //     baton
                //         .data()
                //         .message
                //         .as_string()
                //         .unwrap_or("error null string".to_string())
                // );
                // // data.free_ids();
                // }
            }
        }
    }
    debug!("number of edges in graph: {}", edges.len());
    return edges;
}

// pub unsafe fn physics_update_async(&mut self, updater: CBFnNodeVisitor) -> FFIError {
//     // let mut finished = false;
//     if let Some(force_directed_graph) = &self.force_directed_graph {
//         debug!("fdg exists");

//         let is_finished = force_directed_graph.0.is_finished();

//         if is_finished {
//             let _ = self.force_directed_graph.take().unwrap().0.join();
//             self.force_directed_graph = None;
//             debug!("shutting down physics");
//             return FFIError::PhysicsFinished;
//         } else {
//             debug!("try to update unity");

//             return physics::force_directed_graph::try_update_unity(
//                 &force_directed_graph.1,
//                 updater,
//             );
//         }
//         // let update_result =
//         //     physics::force_directed_graph::try_update_unity(&force_directed_graph.1);
//     }

//     return FFIError::PhysicsAlreadyShutdown;
// }

//creates spring for each edge in graph
fn create_springs(edges_data: Vec<Edge>) -> Vec<Spring> {
    let spring_multiplier = 5.;

    let mut return_vec: Vec<Spring> = Vec::new();

    for data in edges_data {
        //resting length scaled by spring_multiplier
        let new_spring = Spring::new(
            data.2 * spring_multiplier,
            data.0.clone(),
            data.1.clone(),
            data.3,
        );
        return_vec.push(new_spring);
    }

    return_vec
}
