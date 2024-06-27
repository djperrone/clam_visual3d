use std::{
    collections::{HashMap, HashSet},
    f32::consts::PI,
    sync::Arc,
    thread::{self, JoinHandle},
};

use abd_clam::{graph::Edge, Cluster};
use nalgebra::Scalar;
use rand::{seq::SliceRandom, Rng};

use crate::{
    debug,
    ffi_impl::cluster_data::ClusterData,
    graph,
    handle::handle::Handle,
    utils::{
        error::FFIError,
        types::{DataSetf32, Graphf32, Treef32, Vertexf32},
    },
};

type PhysicsEdge = (String, String, f32, bool);

use super::{
    force_directed_graph::ForceDirectedGraph,
    physics_node::PhysicsNode,
    spring::{self, Spring},
};

fn get_k_key_clusters<'a>(
    clam_graph: &'a Graphf32,
    component: &'a HashSet<&'a Vertexf32>,
    k: usize,
) -> Option<Vec<&'a Vertexf32>> {
    let mut key_clusters: Vec<&Vertexf32> = Vec::new();
    let key_cluster = component.iter().max_by(|x, y| {
        clam_graph
            .vertex_degree(x)
            .cmp(&clam_graph.vertex_degree(y))
    });

    if let Some(kc) = key_cluster {
        key_clusters.push(kc);

        let mut comp: Vec<&Vertexf32> = component.iter().map(|x| *x).collect();

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
    key_clusters1: &Vec<&'a Vertexf32>,
    key_clusters2: &Vec<&'a Vertexf32>,
    data: &DataSetf32,
    edges: &mut Vec<Spring>,
    max_edge_len: f32,
    scalar: f32,
) {
    for c1 in key_clusters1.iter() {
        for c2 in key_clusters2.iter() {
            let spring = Spring::new(
                c1.distance_to_other(data, c2),
                c1.name(),
                c2.name(),
                false,
                Some(max_edge_len),
                Some(scalar),
            );
            edges.push(spring);
        }
    }
}

fn create_intercomponent_edges(
    data: &DataSetf32,
    clam_graph: &Graphf32,
    edges: &mut Vec<Spring>,
    k: usize,
    max_edge_len: f32,
    scalar: f32,
) {
    let component_clusters = clam_graph.find_component_clusters();

    for (i, component) in component_clusters.iter().enumerate() {
        if let Some(key_clusters) = get_k_key_clusters(clam_graph, component, k) {
            for component2 in component_clusters.iter().skip(i + 1) {
                if let Some(key_clusters2) = get_k_key_clusters(clam_graph, component2, k) {
                    cross_pollinate_components(
                        &key_clusters,
                        &key_clusters2,
                        data,
                        edges,
                        max_edge_len,
                        scalar,
                    )
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

    let area = PI * clam_graph.ordered_clusters().len() as f32;
    // let area = 100.0f32;
    let max_edge_len = calc_max_edge_len(clam_graph.edges());
    for c in clam_graph.ordered_clusters().iter() {
        let x: f32 = rng.gen_range(0.0..=area);
        let y: f32 = rng.gen_range(0.0..=area);
        let z: f32 = rng.gen_range(0.0..=area);
        graph.insert(c.name(), PhysicsNode::new(glam::Vec3::new(x, y, z), c));
    }
    let mut springs = Vec::new();
    for e in clam_graph.edges() {
        springs.push(Spring::new(
            e.distance(),
            e.left().name(),
            e.right().name(),
            true,
            Some(max_edge_len),
            Some(scalar),
        ));
    }

    create_intercomponent_edges(
        tree.data(),
        clam_graph,
        &mut springs,
        3,
        max_edge_len,
        scalar,
    );

    ForceDirectedGraph::new(graph, springs, scalar, max_iters)
}

fn calc_max_edge_len<'a>(edges: &HashSet<Edge<'a, f32>>) -> f32 {
    match edges
        .iter()
        .reduce(|cur_max: &Edge<'a, f32>, val: &Edge<'a, f32>| {
            if cur_max.distance() > val.distance() {
                cur_max
            } else {
                val
            }
        }) {
        Some(spring) => spring.distance(),
        None => 1.0,
    }

    // max_edge_len
}
