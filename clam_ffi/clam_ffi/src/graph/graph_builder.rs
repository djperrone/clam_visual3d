use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    thread::{self, JoinHandle},
};

use rand::{seq::SliceRandom, Rng};

use crate::{
    debug,
    ffi_impl::cluster_data::ClusterData,
    graph,
    handle::handle::Handle,
    utils::{
        error::FFIError,
        types::{Clusterf32, DataSetf32, Graphf32, Treef32},
    },
};

type Edge = (String, String, f32, bool);

use super::{
    force_directed_graph::ForceDirectedGraph,
    physics_node::PhysicsNode,
    spring::{self, Spring},
};

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
            let spring = Spring::new(c1.distance_to_other(data, c2), c1.name(), c2.name(), false);
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
        if let Some(key_clusters) = get_k_key_clusters(clam_graph, component, k) {
            for component2 in component_clusters.iter().skip(i + 1) {
                if let Some(key_clusters2) = get_k_key_clusters(clam_graph, component2, k) {
                    cross_pollinate_components(&key_clusters, &key_clusters2, data, edges)
                }
            }
        }
    }
}

pub fn build_force_directed_graph_async(
    // cluster_data_arr: &[ClusterData],
    handle: &Handle,
    scalar: f32,
    max_iters: i32,
) -> Result<(JoinHandle<()>, Arc<ForceDirectedGraph>), FFIError> {
    if let Some(tree) = handle.tree() {
        if let Some(clam_graph) = handle.clam_graph() {
            // let mut graph: HashMap<String, PhysicsNode> = HashMap::new();
            // let mut rng = rand::thread_rng();

            // for c in clam_graph.clusters().iter() {
            //     let x: f32 = rng.gen_range(0.0..=100.0);
            //     let y: f32 = rng.gen_range(0.0..=100.0);
            //     let z: f32 = rng.gen_range(0.0..=100.0);
            //     graph.insert(c.name(), PhysicsNode::new(glam::Vec3::new(x, y, z), c));
            // }
            // let mut springs = Vec::new();
            // for e in clam_graph.edges() {
            //     springs.push(Spring::new(
            //         e.distance(),
            //         e.left().name(),
            //         e.right().name(),
            //         true,
            //     ));
            // }

            // create_intercomponent_edges(tree.data(), clam_graph, &mut springs, 3);

            let force_directed_graph = Arc::new(build_force_directed_graph(
                tree, clam_graph, scalar, max_iters,
            ));

            let b = force_directed_graph.clone();
            let p = thread::spawn(move || {
                graph::force_directed_graph::produce_computations(&b);
            });
            return Ok((p, force_directed_graph.clone()));
        }
    }

    Err(FFIError::GraphBuildFailed)
}

pub fn build_force_directed_graph<'a>(
    // cluster_data_arr: &[ClusterData],
    tree: &'a Treef32,
    clam_graph: &'a Graphf32,
    scalar: f32,
    max_iters: i32,
) -> ForceDirectedGraph {
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

    create_intercomponent_edges(tree.data(), clam_graph, &mut springs, 3);

    ForceDirectedGraph::new(graph, springs, scalar, max_iters)
}
