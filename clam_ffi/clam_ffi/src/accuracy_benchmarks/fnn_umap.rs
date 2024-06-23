use std::{
    collections::{BinaryHeap, HashMap},
    fs::{self, ReadDir},
    path::{Path, PathBuf},
};

use abd_clam::{graph::Graph, Cluster, Dataset, PartitionCriteria, Tree};
use glam::Vec3;

use crate::{
    accuracy_benchmarks::utils::calc_fnn_scores,
    handle::handle::Handle,
    utils::{
        distances::DistanceMetric,
        scoring_functions::{enum_to_function, ScoringFunction},
        types::{DataSetf32, Graphf32, Treef32, Vertexf32},
    },
};

use super::{
    false_nearest_neighbors::FNN_Wrapper,
    utils::{self, binary_heap_to_vec},
};

fn test_params(
    single_target: Option<String>,
) -> (
    ReadDir,
    usize,
    usize,
    DistanceMetric,
    f32,
    i32,
    PathBuf,
    String,
    Option<String>,
) {
    let dir_path = Path::new("../../data/anomaly_data/preprocessed");

    // Open the directory
    let data_folder = fs::read_dir(dir_path).unwrap();
    let min_cardinality = 1;
    let min_depth = 11;
    let distance_metric = DistanceMetric::Euclidean;
    let scalar = 100.0;
    let max_iters = 1200;
    let data_folder_name = PathBuf::from(dir_path);

    (
        data_folder,
        min_cardinality,
        min_depth,
        distance_metric,
        scalar,
        max_iters,
        data_folder_name,
        String::from("accuracy_results"),
        single_target,
        // single_target,
    )
}

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

// fn umap_test_fnn_score<'a>(
//     positions: &Vec<Vec3>,
//     tree: &'a Treef32,
//     k: usize,
// ) -> Result<(f64, f64, f64), String> {
//     let mut leaf_clusters: Vec<&'a Vertexf32> = Vec::with_capacity(positions.len());

//     for i in 0..positions.len() {
//         let permuted_index = i;
//         // let original_index = tree.data().original_index(permuted_index);

//         let cluster = tree.get_cluster(permuted_index, 1).unwrap();

//         leaf_clusters.push(&cluster);
//     }
//     let original_nn = find_all_knn_original(&leaf_clusters, tree.data(), k);
//     let umap_nn = find_all_umap_nn(positions, tree, k);

//     for (idx, original_neighbors) in original_nn.iter() {
//         let graph_neighbors = umap_nn.get(idx).unwrap();
//         // Compare original_neighbors with graph_neighbors and calculate metrics
//         let intersection: usize = original_neighbors
//             .iter()
//             .filter(|&n| graph_neighbors.contains(&n))
//             .count();

//         let precision = intersection as f64 / original_neighbors.len() as f64;
//         let recall = intersection as f64 / graph_neighbors.len() as f64;
//         let f1_score = 2.0 * (precision * recall) / (precision + recall);

//         return Ok((precision, recall, f1_score));
//     }
//     Err("error finding umap knn".to_string())
// }

fn run_for_each_umap(
    data_name: &str,
    src_folder: &PathBuf,
    out_folder: &str,
    distance_metric: DistanceMetric,
) {
    match Handle::create_dataset(data_name, &src_folder, distance_metric, false) {
        Ok(data) => {
            println!("created dataset {}", data_name);
            let criteria = PartitionCriteria::new(true).with_min_cardinality(1);

            let tree = Tree::new(data, Some(1)).partition(&criteria, None);
            println!("tree card :{}", tree.cardinality());
            println!("tree data name :{}", tree.data().name());
            let min_depth = {
                if 7 >= tree.depth() {
                    (tree.depth() / 2) as usize
                } else {
                    7
                }
            };
            if let Ok(graph) = Graph::from_tree(
                &tree,
                &enum_to_function(&ScoringFunction::LrEuclideanCc).unwrap(),
                min_depth,
            ) {
                // let cluster_centers = get_cluster_center_indices(&graph, &tree);

                // let dir_path = ;
                // let dir_path = "../../umap/".to_string() + data_name;

                let dir_path = "../../umap/";
                let dir_path = dir_path.to_string() + data_name;
                // Iterate through the directory entries
                if let Ok(entries) = fs::read_dir(dir_path) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            // Get the path of the entry
                            let entry_path = entry.path();

                            // Check if it's a directory
                            if entry_path.is_dir() {
                                // Process the directory
                                println!("Found directory: {}", entry_path.display());
                            } else {
                                if let Some(positions_file) = entry_path.to_str() {
                                    if let Ok(positions) =
                                        utils::read_umap_positions(positions_file)
                                    {
                                        println!("positions file : {}", positions_file);
                                        let graph_k =
                                            utils::extract_umap_k(positions_file).unwrap();
                                        for test_k in 3..20 {
                                            let (original_nn, umap_nn) =
                                                umap_find_knn2(&positions, &graph, &tree, test_k);
                                            let (precision, recall, f1_score) =
                                                calc_fnn_scores(&original_nn, &umap_nn).unwrap();
                                            let mut file_path = PathBuf::new();
                                            // file_path.push("triangle_acc_results");
                                            file_path.push(out_folder);
                                            file_path.push(tree.data().name());
                                            file_path.push(
                                                "graph-k_".to_string()
                                                    + graph_k.to_string().as_str(),
                                            );

                                            file_path.push(
                                                "k_".to_string() + test_k.to_string().as_str(),
                                            );
                                            match fs::create_dir_all(&file_path) {
                                                Ok(_) => {
                                                    println!(
                                                "Folder created successfully or already exists."
                                            )
                                                }
                                                Err(e) => eprintln!("Error creating folder: {}", e),
                                            }

                                            let outfile_name = format!(
                                                "{}_{}_{:?}_{}_precision.csv",
                                                tree.data().name(),
                                                graph_k.to_string(),
                                                distance_metric,
                                                test_k,
                                            );

                                            file_path.push(outfile_name);
                                            utils::write_results(
                                                &file_path,
                                                &vec![precision.to_string()],
                                            );

                                            file_path.pop();

                                            let outfile_name = format!(
                                                "{}_{}_{:?}_{}_recall.csv",
                                                tree.data().name(),
                                                graph_k.to_string(),
                                                distance_metric,
                                                test_k,
                                            );

                                            file_path.push(outfile_name);
                                            println!(
                                                "writing to {:?}",
                                                file_path.to_str().unwrap()
                                            );
                                            utils::write_results(
                                                &file_path,
                                                &vec![recall.to_string()],
                                            );

                                            file_path.pop();

                                            let outfile_name = format!(
                                                "{}_{}_{:?}_{}_f1-score.csv",
                                                tree.data().name(),
                                                graph_k.to_string(),
                                                distance_metric,
                                                test_k,
                                            );

                                            file_path.push(outfile_name);
                                            println!(
                                                "writing to {:?}",
                                                file_path.to_str().unwrap()
                                            );
                                            utils::write_results(
                                                &file_path,
                                                &vec![f1_score.to_string()],
                                            );
                                        }
                                    }
                                    // Process the file
                                    println!("Found file: {}", entry_path.display());
                                }
                            }
                        }
                    }
                }
            } else {
                // eprintln!("Failed to read directory {}", dir_path);
            }
        }
        Err(e) => {
            println!("here {:?}", e);
        }
    }
}
#[test]
fn umap_fnn() {
    let (
        _search_dir,
        _min_cardinality,
        _min_depth,
        distance_metric,
        _scalar,
        _max_iters,
        data_folder,
        out_folder_root,
        _target,
        // ) = test_params(None);
    ) = test_params(None);

    let mut out_folder = PathBuf::new();
    out_folder.push(out_folder_root);
    out_folder.push("umap_fnn");

    let target_datasets = vec![
        // "arrhythmia".to_string(),
        "arrhythmia".to_string(),
        "mnist".to_string(),
        // "wine".to_string(),
        // "satellite".to_string(),
    ];
    for dataset in target_datasets {
        run_for_each_umap(
            dataset.as_str(),
            &data_folder,
            out_folder.to_str().unwrap(),
            distance_metric,
        );
    }
}

// fn get_cluster_center_indices<'a>(graph: &'a Graphf32, tree: &'a Treef32) -> Vec<usize> {
//     let mut centers: Vec<usize> = Vec::with_capacity(graph.ordered_clusters().len());

//     for cluster in graph.ordered_clusters() {
//         let original_index_of_center = tree.data().original_index(cluster.arg_center());
//         centers.push(original_index_of_center);
//     }

//     return centers;
// }

// fn umap_test_centers<'a>(
//     positions: &Vec<Vec3>,
//     graph: &'a Graphf32,
//     tree: &'a Treef32,
//     k: usize,
// ) -> Vec<usize> {
//     let mut centers: Vec<usize> = Vec::with_capacity(graph.ordered_clusters().len());

//     let clusters = graph.ordered_clusters();

//     for &cluster in clusters {
//         let original_nn = knn_naive_original(cluster, clusters, tree.data(), k);
//         let original_index_of_center = tree.data().original_index(cluster.arg_center());
//     }

//     for cluster in graph.ordered_clusters() {
//         let original_index_of_center = tree.data().original_index(cluster.arg_center());
//         centers.push(original_index_of_center);
//     }

//     return centers;
// }

fn umap_find_knn2<'a>(
    positions: &[Vec3],
    graph: &'a Graphf32,
    tree: &'a Treef32,
    k: usize,
) -> (
    HashMap<String, Vec<FNN_Wrapper<'a, f32>>>,
    HashMap<String, Vec<FNN_Wrapper<'a, f32>>>,
) {
    let mut original_nn_map: HashMap<String, Vec<FNN_Wrapper<'a, f32>>> = HashMap::new();
    let mut umap_nn_map: HashMap<String, Vec<FNN_Wrapper<'a, f32>>> = HashMap::new();

    let clusters = graph.ordered_clusters();
    for (i, current_cluster) in clusters.iter().enumerate() {
        let mut original_nn: BinaryHeap<FNN_Wrapper<'a, f32>> = BinaryHeap::new();
        let mut umap_nn: BinaryHeap<FNN_Wrapper<'a, f32>> = BinaryHeap::new();

        let permuted_index_current = current_cluster.arg_center();
        let original_index_current = tree.data().original_index(permuted_index_current);

        for (j, other_cluster) in clusters.iter().enumerate() {
            if i == j {
                continue;
            }

            let permuted_index_other = other_cluster.arg_center();
            let original_index_other = tree.data().original_index(permuted_index_other);

            let original_distance: f32 = distances::vectors::euclidean(
                tree.data().data().get(permuted_index_current).unwrap(),
                tree.data().data().get(permuted_index_other).unwrap(),
            );

            let umap_distance =
                positions[original_index_current].distance(positions[original_index_other]);

            original_nn.push(FNN_Wrapper::new(other_cluster, original_distance));
            umap_nn.push(FNN_Wrapper::new(other_cluster, umap_distance));

            if original_nn.len() > k {
                original_nn.pop();
            }

            if umap_nn.len() > k {
                umap_nn.pop();
            }
        }

        let original_nn_vec = binary_heap_to_vec(original_nn);
        let umap_nn_vec = binary_heap_to_vec(umap_nn);

        original_nn_map.insert(current_cluster.name(), original_nn_vec);
        umap_nn_map.insert(current_cluster.name(), umap_nn_vec);
    }

    return (original_nn_map, umap_nn_map);
}
