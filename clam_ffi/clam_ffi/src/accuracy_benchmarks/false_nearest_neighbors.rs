use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    fmt::Binary,
};

use abd_clam::{chaoda::graph_scorers, graph::Vertex, Cluster};
use distances::Number;
use ndarray::Data;

use crate::{
    graph::force_directed_graph::ForceDirectedGraph,
    utils::types::{DataSetf32, Graphf32, Vertexf32},
};

pub struct FNN_Wrapper<'a, U: Number> {
    cluster: &'a Vertex<U>,
    distance: U,
}

impl<'a, U: Number> FNN_Wrapper<'a, U> {
    pub fn new(cluster: &'a Vertex<U>, distance: U) -> Self {
        FNN_Wrapper { cluster, distance }
    }
}

impl<'a, U: Number> Eq for FNN_Wrapper<'a, U> {}

impl<'a, U: Number> PartialEq for FNN_Wrapper<'a, U> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

// Implementing PartialOrd for PointByX
impl<'a, U: Number> PartialOrd for FNN_Wrapper<'a, U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, U: Number> Ord for FNN_Wrapper<'a, U> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance
            .partial_cmp(&other.distance)
            .unwrap_or(Ordering::Equal)
    }
}

pub fn knn_naive_original<'a>(
    cluster: &'a Vertexf32,
    graph_clusters: &[&'a Vertexf32],
    data: &DataSetf32,
    k: usize,
) -> Vec<&'a Vertexf32> {
    let mut nearest_neighbors: BinaryHeap<FNN_Wrapper<'a, f32>> = BinaryHeap::new();

    for &c in graph_clusters {
        if c.name() != cluster.name() {
            nearest_neighbors.push(FNN_Wrapper::new(c, cluster.distance_to_other(data, c)));

            if nearest_neighbors.len() > k {
                nearest_neighbors.pop();
            }
        }
    }

    let mut result: Vec<&'a Vertexf32> = Vec::with_capacity(k);
    while let Some(wrapper) = nearest_neighbors.pop() {
        result.push(wrapper.cluster);
    }

    // Since the BinaryHeap pops the largest element first, we need to reverse the vector
    result.reverse();

    result
}

pub fn knn_naive_3d<'a>(
    cluster: &'a Vertexf32,
    graph_clusters: &[&'a Vertexf32],
    fdg: &'a ForceDirectedGraph,
    k: usize,
) -> Vec<&'a Vertexf32> {
    let mut nearest_neighbors: BinaryHeap<FNN_Wrapper<'a, f32>> = BinaryHeap::new();
    if let Ok(cluster_position) = fdg.get_cluster_position(&cluster.name()) {
        for &c in graph_clusters {
            if c.name() != cluster.name() {
                let c_3d = fdg.get_cluster_position(&c.name()).unwrap();
                nearest_neighbors.push(FNN_Wrapper::new(c, c_3d.distance(cluster_position)));

                if nearest_neighbors.len() > k {
                    nearest_neighbors.pop();
                }
            }
        }
    }

    let mut result: Vec<&'a Vertexf32> = Vec::with_capacity(k);
    while let Some(wrapper) = nearest_neighbors.pop() {
        result.push(wrapper.cluster);
    }

    // Since the BinaryHeap pops the largest element first, we need to reverse the vector
    result.reverse();

    result
}
pub fn find_all_knn_original<'a>(
    graph: &'a Graphf32,
    data: &DataSetf32,
    k: usize,
) -> HashMap<String, Vec<&'a Vertexf32>> {
    let mut nearest_neighbors_map = HashMap::new();
    let clusters = graph.ordered_clusters();
    for c in clusters {
        nearest_neighbors_map.insert(c.name(), knn_naive_original(c, clusters, data, k));
    }

    nearest_neighbors_map
}

pub fn find_all_knn_3d<'a>(
    fdg: &'a ForceDirectedGraph,
    clam_graph: &'a Graphf32,
    k: usize,
) -> HashMap<String, Vec<&'a Vertexf32>> {
    let mut nearest_neighbors_map = HashMap::new();
    let clusters = clam_graph.ordered_clusters();
    for c in clusters {
        nearest_neighbors_map.insert(c.name(), knn_naive_3d(c, clusters, fdg, k));
    }

    nearest_neighbors_map
}

pub fn false_nearest_neighbors(
    clam_graph: &Graphf32,
    data: &DataSetf32,
    fdg: &ForceDirectedGraph,
    k: usize,
) {
    let original_nn = find_all_knn_original(clam_graph, data, k);
    let fdg_nn = find_all_knn_3d(fdg, clam_graph, k);

    for (idx, original_neighbors) in original_nn.iter() {
        let graph_neighbors = fdg_nn.get(idx).unwrap();
        // Compare original_neighbors with graph_neighbors and calculate metrics
        let intersection: usize = original_neighbors
            .iter()
            .filter(|&&n| graph_neighbors.contains(&n))
            .count();

        let precision = intersection as f64 / original_neighbors.len() as f64;
        let recall = intersection as f64 / graph_neighbors.len() as f64;
        let f1_score = 2.0 * (precision * recall) / (precision + recall);

        println!(
            "Data point {}: Precision={}, Recall={}, F1-score={}",
            idx, precision, recall, f1_score
        );
    }
}
