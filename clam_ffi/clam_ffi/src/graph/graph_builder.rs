use std::{
    collections::HashMap,
    sync::Arc,
    thread::{self, JoinHandle},
};

use rand::Rng;

use crate::{
    debug,
    ffi_impl::cluster_data::ClusterData,
    graph,
    handle::handle::Handle,
    utils::{
        error::FFIError,
        types::{Clusterf32, DataSetf32},
    },
};

type Edge = (String, String, f32, bool);

use super::{
    force_directed_graph::ForceDirectedGraph,
    physics_node::PhysicsNode,
    spring::{self, Spring},
};

pub unsafe fn build_force_directed_graph(
    // cluster_data_arr: &[ClusterData],
    handle: &Handle,
    scalar: f32,
    max_iters: i32,
) -> Result<(JoinHandle<()>, Arc<ForceDirectedGraph>), FFIError> {
    // let springs: Vec<Spring> = {
    //     // let mut clusters: Vec<&Clusterf32> = Vec::new();

    //     // for c in cluster_data_arr.iter() {
    //     //     if let Ok(cluster) = handle.get_cluster_from_string(c.id.as_string().unwrap()) {
    //     //         clusters.push(cluster);
    //     //     }
    //     // }
    //     // create_springs(detect_edges(&clusters, &handle.data())) //, edge_detector_cb))
    // };
    if let Some(_) = handle.get_tree() {
        if let Some(clam_graph) = handle.clam_graph() {
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

            let force_directed_graph =
                Arc::new(ForceDirectedGraph::new(graph, springs, scalar, max_iters));

            let b = force_directed_graph.clone();
            let p = thread::spawn(move || {
                graph::force_directed_graph::produce_computations(&b);
            });
            return Ok((p, force_directed_graph.clone()));
        }
    }

    Err(FFIError::GraphBuildFailed)
    // let graph = build_graph(handle, cluster_data_arr);
    // if graph.is_empty() {
    //     return Err(FFIError::GraphBuildFailed);
    // }
}

// pub unsafe fn build_graph(
//     handle: &Handle,
//     // cluster_data_arr: &[ClusterData],
// ) -> HashMap<String, PhysicsNode> {
//     let mut graph: HashMap<String, PhysicsNode> = HashMap::new();

//     for c in cluster_data_arr {
//         graph.insert(
//             c.id.as_string().unwrap(),
//             PhysicsNode::new(
//                 c,
//                 handle
//                     .get_cluster_from_string(c.id.as_string().unwrap())
//                     .unwrap(),
//             ),
//         );
//     }

//     graph
// }

// pub fn detect_edges(clusters: &Vec<&Clusterf32>, dataset: &Option<&DataSetf32>) -> Vec<Edge> {
//     let mut edges: Vec<Edge> = Vec::new();
//     if let Some(data) = *dataset {
//         for i in 0..clusters.len() {
//             for j in (i + 1)..clusters.len() {
//                 let distance = clusters[i].distance_to_other(data, clusters[j]);
//                 if distance <= clusters[i].radius() + clusters[j].radius() {
//                     edges.push((clusters[i].name(), clusters[j].name(), distance, true));
//                 }
//             }
//         }
//     } else {
//         debug!("error detect edges no dataset found");
//     }
//     edges
// }

// //creates spring for each edge in graph
// fn create_springs(edges_data: Vec<Edge>) -> Vec<Spring> {
//     let spring_multiplier = 5.;

//     let mut return_vec: Vec<Spring> = Vec::new();

//     for data in &edges_data {
//         //resting length scaled by spring_multiplier
//         let new_spring = Spring::new(
//             data.2 * spring_multiplier,
//             data.0.clone(),
//             data.1.clone(),
//             data.3,
//         );
//         return_vec.push(new_spring);
//     }
//     return_vec
// }
