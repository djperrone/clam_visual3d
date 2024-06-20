use std::collections::{BinaryHeap, HashMap};

use abd_clam::{Cluster, Dataset};
use glam::Vec3;

use crate::utils::types::{DataSetf32, Treef32, Vertexf32};

use super::false_nearest_neighbors::{find_all_knn_original, knn_naive_original, FNN_Wrapper};

fn umap_find_knn<'a>(
    positions: &[Vec3],
    chosen_original_index: usize,
    tree: &'a Treef32,
    k: usize,
) -> Vec<FNN_Wrapper<'a, f32>> {
    let mut nearest_neighbors: BinaryHeap<FNN_Wrapper<'a, f32>> = BinaryHeap::new();

    for permuted_i in 0..positions.len() {
        let original_index = tree.data().original_index(permuted_i);
        if chosen_original_index != original_index {
            nearest_neighbors.push(FNN_Wrapper::new(
                tree.get_cluster(permuted_i, 1).unwrap(),
                positions[original_index].distance(positions[chosen_original_index]),
            ));
        }

        if nearest_neighbors.len() > k {
            nearest_neighbors.pop();
        }
    }

    let mut result: Vec<FNN_Wrapper<'a, f32>> = Vec::with_capacity(k);
    while let Some(wrapper) = nearest_neighbors.pop() {
        result.push(wrapper);
    }

    // Since the BinaryHeap pops the largest element first, we need to reverse the vector
    result.reverse();

    result
}

// pub fn find_all_knn_original<'a>(
//     clusters: &[&'a Vertexf32],
//     data: &DataSetf32,
//     k: usize,
// ) -> HashMap<String, Vec<FNN_Wrapper<'a, f32>>> {
//     let mut nearest_neighbors_map = HashMap::new();
//     for c in clusters {
//         nearest_neighbors_map.insert(c.name(), knn_naive_original(c, clusters, data, k));
//     }

//     nearest_neighbors_map
// }

pub fn find_all_umap_nn<'a>(
    positions: &[Vec3],
    tree: &'a Treef32,
    k: usize,
) -> HashMap<String, Vec<FNN_Wrapper<'a, f32>>> {
    let mut nearest_neighbors_map = HashMap::new();
    for permuted_i in 0..positions.len() {
        let original_index = tree.data().original_index(permuted_i);
        let chosen_cluster = tree.get_cluster(permuted_i, 1).unwrap();
        nearest_neighbors_map.insert(
            chosen_cluster.name(),
            umap_find_knn(positions, original_index, tree, k),
        );
    }

    nearest_neighbors_map
}

fn umap_find_knn_all<'a>(positions: &Vec<Vec3>, tree: &'a Treef32, k: usize) -> () {
    let mut leaf_clusters: Vec<&'a Vertexf32> = Vec::with_capacity(positions.len());

    for i in 0..positions.len() {
        let permuted_index = i;
        // let original_index = tree.data().original_index(permuted_index);

        let cluster = tree.get_cluster(permuted_index, 1).unwrap();

        leaf_clusters.push(&cluster);
    }
    let original_nn = find_all_knn_original(&leaf_clusters, tree.data(), k);
    let umap_nn = find_all_umap_nn(positions, tree, k);

    for (idx, original_neighbors) in original_nn.iter() {
        let graph_neighbors = umap_nn.get(idx).unwrap();
        // Compare original_neighbors with graph_neighbors and calculate metrics
        let intersection: usize = original_neighbors
            .iter()
            .filter(|&n| graph_neighbors.contains(&n))
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
