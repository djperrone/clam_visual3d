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
    // graph::{force_directed_graph::ForceDirectedGraph, graph_builder::build_force_directed_graph},
    h_graph::{self, h_graph::ForceDirectedGraph},
    handle::handle::Handle,
    utils::{
        distances::DistanceMetric,
        scoring_functions::{enum_to_function, ScoringFunction},
        types::{DataSetf32, Graphf32, Treef32, Vertexf32},
    },
};

use super::{false_nearest_neighbors::FNN_Wrapper, utils::binary_heap_to_vec};

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

#[test]
fn h_graph_test() {
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
        "wine".to_string(),
        "arrhythmia".to_string(),
        // "satellite".to_string(),
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
            for depth in 4..(10) as usize {
                if let Ok(graph) = Graph::from_tree(
                    &tree,
                    &enum_to_function(&ScoringFunction::LrEuclideanCc).unwrap(),
                    depth,
                ) {
                    if let Ok(fdg) = run_physics_sim(&tree, &graph, scalar, max_iters) {
                        for k in 3..15 {
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
    let mut fdg =
        h_graph::h_graph::ForceDirectedGraph::new(tree, &graph, scalar, max_iters as usize)?;
    println!("running new physics");
    fdg.run_physics(tree, graph)?;

    // let mut fdg =
    //     h_graph::h_graph::ForceDirectedGraph::old_new(tree, &graph, scalar, max_iters as usize)?;

    // fdg.run_old_physics(tree, graph);
    // let mut fdg = build_force_directed_graph(&tree, &graph, scalar, max_iters);
    println!("created fdg");

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
