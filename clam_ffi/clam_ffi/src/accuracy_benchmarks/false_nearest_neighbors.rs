use core::panic;
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    env,
    fmt::Binary,
    fs::{self, ReadDir},
    path::{Path, PathBuf},
};

use abd_clam::{
    chaoda::graph_scorers,
    graph::{Graph, Vertex},
    Cluster, Dataset, PartitionCriteria, Tree,
};
use distances::Number;
use glam::Vec3;
use nalgebra::max;
use ndarray::Data;

use crate::{
    accuracy_benchmarks::utils::{self, calc_fnn_scores},
    graph::{force_directed_graph::ForceDirectedGraph, graph_builder::build_force_directed_graph},
    handle::handle::Handle,
    utils::{
        distances::DistanceMetric,
        scoring_functions::{enum_to_function, ScoringFunction},
        types::{DataSetf32, Graphf32, Treef32, Vertexf32},
    },
};

use super::utils::binary_heap_to_vec;

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

// Implementing PartialEq based on name
impl<'a, U: Number> PartialEq for FNN_Wrapper<'a, U> {
    fn eq(&self, other: &Self) -> bool {
        self.cluster.name() == other.cluster.name()
    }
}

// Implementing PartialOrd based on distance
impl<'a, U: Number> PartialOrd for FNN_Wrapper<'a, U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

// Implementing Ord based on distance, with name as tie-breaker
impl<'a, U: Number> Ord for FNN_Wrapper<'a, U> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.distance.partial_cmp(&other.distance) {
            Some(Ordering::Equal) => self.cluster.name().cmp(&other.cluster.name()),
            Some(ordering) => ordering,
            None => Ordering::Equal, // Handle NaN case or any other undefined behavior
        }
    }
}

// impl<'a, U: Number> Ord for FNN_Wrapper<'a, U> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.distance
//             .partial_cmp(&other.distance)
//             .unwrap_or(Ordering::Equal)
//     }
// }

// pub fn knn_naive_original<'a>(
//     cluster: &'a Vertexf32,
//     graph_clusters: &[&'a Vertexf32],
//     data: &DataSetf32,
//     k: usize,
// ) -> Vec<FNN_Wrapper<'a, f32>> {
//     let mut nearest_neighbors: BinaryHeap<FNN_Wrapper<'a, f32>> = BinaryHeap::new();

//     for &c in graph_clusters {
//         if c.name() != cluster.name() {
//             nearest_neighbors.push(FNN_Wrapper::new(c, cluster.distance_to_other(data, c)));

//             if nearest_neighbors.len() > k {
//                 nearest_neighbors.pop();
//             }
//         }
//     }

//     let mut result: Vec<FNN_Wrapper<'a, f32>> = Vec::with_capacity(k);
//     while let Some(wrapper) = nearest_neighbors.pop() {
//         result.push(wrapper);
//     }

//     // Since the BinaryHeap pops the largest element first, we need to reverse the vector
//     result.reverse();

//     result
// }

// pub fn knn_naive_3d<'a>(
//     cluster: &'a Vertexf32,
//     graph_clusters: &[&'a Vertexf32],
//     fdg: &'a ForceDirectedGraph,
//     k: usize,
// ) -> Vec<FNN_Wrapper<'a, f32>> {
//     let mut nearest_neighbors: BinaryHeap<FNN_Wrapper<'a, f32>> = BinaryHeap::new();
//     if let Ok(cluster_position) = fdg.get_cluster_position(&cluster.name()) {
//         for &c in graph_clusters {
//             if c.name() != cluster.name() {
//                 let other_pos = fdg.get_cluster_position(&c.name()).unwrap();
//                 nearest_neighbors.push(FNN_Wrapper::new(c, other_pos.distance(cluster_position)));

//                 if nearest_neighbors.len() > k {
//                     nearest_neighbors.pop();
//                 }
//             }
//         }
//     }

//     let mut result: Vec<FNN_Wrapper<'a, f32>> = Vec::with_capacity(k);
//     while let Some(wrapper) = nearest_neighbors.pop() {
//         result.push(wrapper);
//     }

//     // Since the BinaryHeap pops the largest element first, we need to reverse the vector
//     result.reverse();

//     result
// }
// pub fn find_all_knn_original<'a>(
//     clusters: &[&'a Vertexf32],
//     data: &DataSetf32,
//     k: usize,
// ) -> HashMap<String, Vec<FNN_Wrapper<'a, f32>>> {
//     let mut nearest_neighbors_map = HashMap::new();
//     // let clusters = graph.ordered_clusters();
//     for c in clusters {
//         nearest_neighbors_map.insert(c.name(), knn_naive_original(c, clusters, data, k));
//     }

//     nearest_neighbors_map
// }

// pub fn find_all_knn_3d<'a>(
//     fdg: &'a ForceDirectedGraph,
//     clusters: &[&'a Vertexf32],
//     k: usize,
// ) -> HashMap<String, Vec<FNN_Wrapper<'a, f32>>> {
//     let mut nearest_neighbors_map = HashMap::new();
//     for c in clusters {
//         nearest_neighbors_map.insert(c.name(), knn_naive_3d(c, clusters, fdg, k));
//     }

//     nearest_neighbors_map
// }

// pub fn false_nearest_neighbors(
//     clam_graph: &Graphf32,
//     data: &DataSetf32,
//     fdg: &ForceDirectedGraph,
//     k: usize,
// ) {
//     let original_nn = find_all_knn_original(clam_graph.ordered_clusters(), data, k);
//     let fdg_nn = find_all_knn_3d(fdg, clam_graph.ordered_clusters(), k);

//     for (idx, original_neighbors) in original_nn.iter() {
//         let graph_neighbors = fdg_nn.get(idx).unwrap();
//         // Compare original_neighbors with graph_neighbors and calculate metrics
//         let intersection: usize = original_neighbors
//             .iter()
//             .filter(|&n| graph_neighbors.contains(&n))
//             .count();

//         let precision = intersection as f64 / original_neighbors.len() as f64;
//         let recall = intersection as f64 / graph_neighbors.len() as f64;
//         let f1_score = 2.0 * (precision * recall) / (precision + recall);

//         println!(
//             "Data point {}: Precision={}, Recall={}, F1-score={}",
//             idx, precision, recall, f1_score
//         );
//     }
// }

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
    // let k: usize = ;

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

fn run_test_on_file(
    filename: &str,
    src_folder: &PathBuf,
    out_folder: &str,
    distance_metric: DistanceMetric,
    min_cardinality: usize,
    max_iters: i32,
    scalar: f32,
) {
    match Handle::create_dataset(filename, &src_folder, distance_metric, false) {
        Ok(data) => {
            println!("created dataset");
            let criteria = PartitionCriteria::new(true).with_min_cardinality(min_cardinality);

            let tree = Tree::new(data, Some(1)).partition(&criteria, None);
            for depth in 4..(tree.depth().as_f32() * 0.75) as usize {
                if let Ok(graph) = Graph::from_tree(
                    &tree,
                    &enum_to_function(&ScoringFunction::LrEuclideanCc).unwrap(),
                    depth,
                ) {
                    if let Ok(fdg) = run_physics_sim(&tree, &graph, scalar, max_iters) {
                        for k in 3..20 {
                            println!("created graph");
                            let outfile_name = format!(
                                "{}_{}_{:?}_{}.csv",
                                tree.data().name(),
                                min_cardinality.to_string(),
                                distance_metric,
                                k,
                            );
                            let descriptor_file = format!(
                                "{}_{}_{:?}_{}.txt",
                                tree.data().name(),
                                min_cardinality.to_string(),
                                distance_metric,
                                k,
                            );
                            let (original_nn, fdg_nn) = clam_find_knn2(&fdg, &graph, &tree, k);
                            let (precision, recall, f1_score) =
                                calc_fnn_scores(&original_nn, &fdg_nn).unwrap();
                            let mut file_path = PathBuf::new();
                            // file_path.push("triangle_acc_results");
                            file_path.push(out_folder);
                            file_path.push(tree.data().name());
                            file_path.push("depth_".to_string() + depth.to_string().as_str());

                            file_path.push("k_".to_string() + k.to_string().as_str());
                            match fs::create_dir_all(&file_path) {
                                Ok(_) => {
                                    println!("Folder created successfully or already exists.")
                                }
                                Err(e) => eprintln!("Error creating folder: {}", e),
                            }

                            let outfile_name = format!(
                                "{}_{}_{:?}_{}_precision.csv",
                                tree.data().name(),
                                min_cardinality.to_string(),
                                distance_metric,
                                k,
                            );
                            file_path.push(outfile_name);

                            println!("writing to {:?}", file_path.to_str().unwrap());
                            utils::write_results(&file_path, &vec![precision.to_string()]);

                            file_path.pop();

                            let outfile_name = format!(
                                "{}_{}_{:?}_{}_recall.csv",
                                tree.data().name(),
                                min_cardinality.to_string(),
                                distance_metric,
                                k,
                            );

                            file_path.push(outfile_name);
                            println!("writing to {:?}", file_path.to_str().unwrap());
                            utils::write_results(&file_path, &vec![recall.to_string()]);

                            file_path.pop();

                            let outfile_name = format!(
                                "{}_{}_{:?}_{}_f1-score.csv",
                                tree.data().name(),
                                min_cardinality.to_string(),
                                distance_metric,
                                k,
                            );

                            file_path.push(outfile_name);
                            println!("writing to {:?}", file_path.to_str().unwrap());
                            utils::write_results(&file_path, &vec![f1_score.to_string()]);

                            file_path.pop();
                            file_path.push(descriptor_file);
                            let descriptors = vec![
                                "data_cardinality".to_string(),
                                "graph_vertex_cardinality".to_string(),
                                "graph_edge_cardinality".to_string(),
                                "tree_height".to_string(),
                            ];
                            let descriptor_data = vec![
                                tree.data().cardinality().to_string(),
                                graph.vertex_cardinality().to_string(),
                                graph.edge_cardinality().to_string(),
                                tree.depth().to_string(),
                            ];
                            utils::write_results(&file_path, &descriptors);
                            utils::write_results(&file_path, &descriptor_data);
                        }
                    } else {
                        panic!("collecting data for this graph failed");
                    }
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}

fn run_physics_sim(
    tree: &Treef32,
    graph: &Graphf32,
    scalar: f32,
    max_iters: i32,
) -> Result<ForceDirectedGraph, String> {
    let mut fdg = build_force_directed_graph(&tree, &graph, scalar, max_iters, true);
    println!("created fdg");
    // let mut precision_results: Vec<String> = Vec::with_capacity(max_iters as usize);
    // let mut recall_results: Vec<String> = Vec::with_capacity(max_iters as usize);
    // let mut f1_score_results: Vec<String> = Vec::with_capacity(max_iters as usize);

    // let original_nn = find_all_knn_original(graph.ordered_clusters(), tree.data(), k);
    for i in 0..max_iters {
        if i % 100 == 0 {
            println!("time step {}", i);
        }

        fdg.update(&graph, &tree);

        // match run_fnn_test(&graph, &fdg, &original_nn, k) {
        //     Ok(accuracy) => {
        //         precision_results.push(accuracy.0.to_string());
        //         recall_results.push(accuracy.1.to_string());
        //         f1_score_results.push(accuracy.2.to_string());
        //     }
        //     Err(e) => {
        //         //return Err(e);
        //         panic!();
        //     }
        // }
    }
    Ok(fdg)

    // Ok((precision_results, recall_results, f1_score_results))
}

// pub fn run_fnn_test(
//     clam_graph: &Graphf32,
//     fdg: &ForceDirectedGraph,
//     original_nn: &HashMap<String, Vec<FNN_Wrapper<f32>>>,
//     k: usize,
// ) -> Result<(f64, f64, f64), String> {
//     if clam_graph.ordered_clusters().len() < 3 {
//         return Err("less than 3 clusters in graph".to_string());
//     }

//     let fdg_nn = find_all_knn_3d(fdg, clam_graph.ordered_clusters(), k);

//     for (idx, original_neighbors) in original_nn.iter() {
//         let graph_neighbors = fdg_nn.get(idx).unwrap();
//         // Compare original_neighbors with graph_neighbors and calculate metrics
//         let intersection: usize = original_neighbors
//             .iter()
//             .filter(|&n| graph_neighbors.contains(&n))
//             .count();

//         let precision = intersection as f64 / original_neighbors.len() as f64;
//         let recall = intersection as f64 / graph_neighbors.len() as f64;
//         let f1_score = 2.0 * (precision * recall) / (precision + recall);

//         // println!(
//         //     "Data point {}: Precision={}, Recall={}, F1-score={}",
//         //     idx, precision, recall, f1_score
//         // );

//         return Ok((precision, recall, f1_score));
//     }

//     return Err("something went wrong".to_string());
// }

fn run_single_target(
    filename: &str,
    src_folder: &PathBuf,
    out_folder: &str,
    distance_metric: DistanceMetric,
    min_cardinality: usize,
    max_iters: i32,
    scalar: f32,
) {
    run_test_on_file(
        filename,
        &src_folder,
        out_folder,
        distance_metric,
        min_cardinality,
        max_iters,
        scalar,
    );
}

fn run_for_each(
    dir: ReadDir,
    min_cardinality: usize,
    min_depth: usize,
    distance_metric: DistanceMetric,
    scalar: f32,
    max_iters: i32,
    src_folder: &PathBuf,
    outfolder: &str,
    single_target: Option<String>,
    k: usize,
) {
    if let Ok(current_dir) = env::current_dir() {
        // If successful, print the current working directory
        println!("Current working directory: {}", current_dir.display());
    } else {
        // If there was an error getting the current directory, print an error message
        eprintln!("Failed to get the current working directory");
    }
    println!("src folder {}", src_folder.to_str().unwrap());
    let mut finished: HashSet<String> = HashSet::new();
    // Iterate through each entry in the directory
    for filename in dir {
        if let Ok(filename) = filename {
            // Convert the file name to a string
            if let Some(filename) = filename.file_name().to_str() {
                let parts: Vec<&str> = filename.split('_').collect();
                println!("file: {}", filename);
                // Check if the split operation produced at least one part
                if let Some(&filename) = parts.first() {
                    if !finished.contains(&filename.to_string()) {
                        println!("filename: {}", filename);
                        // if *filename == "smtp" {
                        if let Some(target) = &single_target {
                            if target == filename {
                                run_test_on_file(
                                    filename,
                                    &src_folder,
                                    outfolder,
                                    distance_metric,
                                    min_cardinality,
                                    max_iters,
                                    scalar,
                                );
                                break;
                            }
                        } else {
                            run_test_on_file(
                                filename,
                                &src_folder,
                                outfolder,
                                distance_metric,
                                min_cardinality,
                                max_iters,
                                scalar,
                            );
                        }

                        // }
                        finished.insert(filename.to_string());
                    }
                }
            }
        }
    }
}

#[test]
fn fnn() {
    // for depth in 4..12 {
    // build tree/graph here
    // for k in 3..50 {
    let (
        dir,
        min_cardinality,
        min_depth,
        distance_metric,
        scalar,
        max_iters,
        src_folder,
        out_folder_root,
        target,
        // ) = test_params(None);
    ) = test_params(Some("wine".to_string()));

    // let outfolder = "angle_distortion";
    let mut out_folder = PathBuf::new();
    out_folder.push(out_folder_root);
    out_folder.push("fnn");

    let targets = vec![
        // "arrhythmia".to_string(),
        // "satellite".to_string(),
        "wine".to_string(),
    ];

    for target in targets {
        run_single_target(
            &target,
            &src_folder,
            out_folder.to_str().unwrap(),
            distance_metric,
            min_cardinality,
            max_iters,
            scalar,
        )
    }

    // run_for_each(
    //     dir,
    //     min_cardinality,
    //     min_depth,
    //     distance_metric,
    //     scalar,
    //     max_iters,
    //     &src_folder,
    //     out_folder.to_str().unwrap(),
    //     target,
    //     k,
    // );
}
// }
// }

// #[test]
// fn umap_test_ang1le_distortion() {
//     let (
//         search_dir,
//         _min_cardinality,
//         _min_depth,
//         distance_metric,
//         _scalar,
//         _max_iters,
//         data_folder,
//         out_folder_root,
//         target,
//         // ) = test_params(None);
//     ) = test_params(None);

//     let mut out_folder = PathBuf::new();
//     out_folder.push(out_folder_root);
//     out_folder.push("umap_fnn");
//     // let metric_cb = utils::calc_angle_distortion;

//     run_for_each_umap(
//         search_dir,
//         target.unwrap().as_str(),
//         &data_folder,
//         out_folder.to_str().unwrap(),
//         distance_metric,
//         k,
//     );
// }

// fn run_for_each_umap(
//     dir: ReadDir,
//     data_name: &str,
//     src_folder: &PathBuf,
//     outfolder: &str,
//     distance_metric: DistanceMetric,
//     k: usize,
// ) {
//     match Handle::create_dataset(data_name, &src_folder, distance_metric, false) {
//         Ok(data) => {
//             println!("created dataset {}", data_name);
//             let criteria = PartitionCriteria::new(true).with_min_cardinality(1);

//             let tree = Tree::new(data, Some(1)).partition(&criteria, None);
//             println!("tree card :{}", tree.cardinality());
//             println!("tree data name :{}", tree.data().name());
//             // let dir_path = ;
//             // let dir_path = "../../umap/".to_string() + data_name;

//             let mut scores: HashMap<u32, Vec<f64>> = HashMap::new();
//             for _ in 0..5 {
//                 let dir_path = "../../umap/";
//                 let dir_path = dir_path.to_string() + data_name;
//                 // Iterate through the directory entries
//                 if let Ok(entries) = fs::read_dir(dir_path) {
//                     for entry in entries {
//                         if let Ok(entry) = entry {
//                             // Get the path of the entry
//                             let entry_path = entry.path();

//                             // Check if it's a directory
//                             if entry_path.is_dir() {
//                                 // Process the directory
//                                 println!("Found directory: {}", entry_path.display());
//                             } else {
//                                 if let Some(positions_file) = entry_path.to_str() {
//                                     println!("positions file : {}", positions_file);
//                                     let umap_k =
//                                         utils::extract_umap_k(positions_file).unwrap() as usize;

//                                     let avg =
//                                         run_umap_test_on_file(positions_file, &tree, metric_cb)
//                                             .unwrap();
//                                     println!("avg: {}", avg);
//                                     if !scores.contains_key(&k) {
//                                         scores.insert(k, Vec::new());
//                                     }
//                                     scores.get_mut(&k).unwrap().push(avg);
//                                 }
//                                 // Process the file
//                                 println!("Found file: {}", entry_path.display());
//                             }
//                         }
//                     }
//                 } else {
//                     // eprintln!("Failed to read directory {}", dir_path);
//                 }
//             }

//             println!("scores: {:?}", scores);
//             let mut outpath = PathBuf::new();
//             outpath.push(outfolder);
//             fs::create_dir_all(&outpath);

//             outpath.push(data_name.to_string() + ".csv");
//             // outpath.push(".csv");
//             println!("ouptath {:?}", outpath);
//             let mut avg_scores: HashMap<u32, f64> = HashMap::new();
//             for (key, value) in scores {
//                 let avg: f64 = value.iter().sum::<f64>() / value.len().as_f64();
//                 avg_scores.insert(key, avg);
//             }
//             utils::write_umap_scores_to_file(outpath.to_str().unwrap(), &avg_scores);
//         }
//         Err(e) => {
//             println!("{:?}", e);
//         }
//     }
// }

// fn umap_find_knn<'a>(
//     positions: &Vec<Vec3>,
//     original_index: usize,
//     cluster: &Vertexf32,
//     k: usize,
// ) -> Vec<FNN_Wrapper<'a, f32>> {
//     let mut nearest_neighbors: BinaryHeap<FNN_Wrapper<'a, f32>> = BinaryHeap::new();

//     for i in 0..positions.len() {
//         if i != original_index {
//             nearest_neighbors.push(FNN_Wrapper::new(
//                 cluster,
//                 positions[i].distance(positions[original_index]),
//             ));

//             if nearest_neighbors.len() > k {
//                 nearest_neighbors.pop();
//             }
//         }
//     }

//     let mut result: Vec<FNN_Wrapper<'a, f32>> = Vec::with_capacity(k);
//     while let Some(wrapper) = nearest_neighbors.pop() {
//         result.push(wrapper);
//     }

//     // Since the BinaryHeap pops the largest element first, we need to reverse the vector
//     result.reverse();

//     result
// }

// fn run_umap_fnn_on_file(file_path: &str, tree: &Treef32, k: usize) -> () {
//     let mut valid_count = 0;
//     let mut metric_sum: f64 = 0.;

//     if let Ok(positions) = utils::read_umap_positions(file_path) {
//         let data = tree.data();

//         let mut cluster_pool: Vec<&Vertexf32> = Vec::with_capacity(positions.len());

//         for reordered_index in 0..positions.len() {
//             cluster_pool.push(tree.get_cluster(reordered_index, 1).unwrap());
//         }

//         for reordered_index in 0..positions.len() {
//             // println!("test1");
//             let original_index = data.original_index(reordered_index as usize);
//             // println!("test2");

//             let selected_position = positions[original_index];
//             let cluster = tree.get_cluster(reordered_index, 1).unwrap();

//             let umap_nn = umap_find_knn(&positions, original_index, cluster, k);
//             // results.push(perc_correct);
//         }
//     }
// }

fn clam_find_knn2<'a>(
    fdg: &ForceDirectedGraph,
    graph: &'a Graphf32,
    tree: &'a Treef32,
    k: usize,
) -> (
    HashMap<String, Vec<FNN_Wrapper<'a, f32>>>,
    HashMap<String, Vec<FNN_Wrapper<'a, f32>>>,
) {
    let mut original_nn_map: HashMap<String, Vec<FNN_Wrapper<'a, f32>>> = HashMap::new();
    let mut fdg_nn_map: HashMap<String, Vec<FNN_Wrapper<'a, f32>>> = HashMap::new();

    let clusters = graph.ordered_clusters();
    for (i, current_cluster) in clusters.iter().enumerate() {
        let mut original_nn: BinaryHeap<FNN_Wrapper<'a, f32>> = BinaryHeap::new();
        let mut fdg_nn: BinaryHeap<FNN_Wrapper<'a, f32>> = BinaryHeap::new();

        // let permuted_index_current = current_cluster.arg_center();
        // let original_index_current = tree.data().original_index(permuted_index_current);

        for (j, other_cluster) in clusters.iter().enumerate() {
            if i == j {
                continue;
            }

            let original_distance: f32 =
                current_cluster.distance_to_other(tree.data(), &other_cluster);

            let fdg_distance = fdg
                .get_cluster_position(&current_cluster.name())
                .unwrap()
                .distance(fdg.get_cluster_position(&other_cluster.name()).unwrap());

            original_nn.push(FNN_Wrapper::new(other_cluster, original_distance));
            fdg_nn.push(FNN_Wrapper::new(other_cluster, fdg_distance));

            if original_nn.len() > k {
                original_nn.pop();
            }

            if fdg_nn.len() > k {
                fdg_nn.pop();
            }
        }

        let original_nn_vec = binary_heap_to_vec(original_nn);
        let fdg_nn_vec = binary_heap_to_vec(fdg_nn);

        original_nn_map.insert(current_cluster.name(), original_nn_vec);
        fdg_nn_map.insert(current_cluster.name(), fdg_nn_vec);
    }

    return (original_nn_map, fdg_nn_map);
}
