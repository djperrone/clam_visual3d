use std::{
    collections::HashSet,
    env,
    fs::{self, ReadDir},
    io::Read,
    path::{Path, PathBuf},
};

use abd_clam::{Dataset, Graph, PartitionCriteria, Tree};
use distances::Number;
use rand::seq::SliceRandom;

use crate::{
    graph::{force_directed_graph::ForceDirectedGraph, graph_builder::build_force_directed_graph},
    handle::handle::Handle,
    utils::{
        distances::DistanceMetric,
        scoring_functions::{enum_to_function, ScoringFunction},
        types::{Graphf32, Treef32},
    },
};

use super::utils;

fn test_params() -> (
    ReadDir,
    usize,
    usize,
    DistanceMetric,
    f32,
    i32,
    PathBuf,
    Option<String>,
) {
    let dir_path = Path::new("../../data/anomaly_data/preprocessed");

    // Open the directory
    let data_folder = fs::read_dir(dir_path).unwrap();
    let min_cardinality = 50;
    let min_depth = 8;
    let distance_metric = DistanceMetric::Euclidean;
    let scalar = 100.0;
    let max_iters = 1000;
    let data_folder_name = PathBuf::from(dir_path);

    (
        data_folder,
        min_cardinality,
        min_depth,
        distance_metric,
        scalar,
        max_iters,
        data_folder_name,
        Some("smtp".to_string()),
    )
}

fn run_test_on_file(
    filename: &str,
    src_folder: &PathBuf,
    out_folder: &str,
    distance_metric: DistanceMetric,
    min_cardinality: usize,
    min_depth: usize,
    max_iters: i32,
    scalar: f32,
    test_cb: fn(&mut Vec<(&str, f32)>, &mut Vec<(&str, f32)>) -> f64,
) {
    match Handle::create_dataset(filename, &src_folder, distance_metric, false) {
        Ok(data) => {
            println!("created dataset");
            let criteria = PartitionCriteria::new(true).with_min_cardinality(min_cardinality);

            let tree = Tree::new(data, Some(1))
                .partition(&criteria)
                .with_ratios(false);

            if let Ok(graph) = Graph::from_tree(
                &tree,
                &enum_to_function(&ScoringFunction::LrEuclideanCc).unwrap(),
                min_depth,
            ) {
                println!("created graph");
                let outfile_name = format!(
                    "{}_{}_{:?}_{}.csv",
                    tree.data().name(),
                    min_cardinality.to_string(),
                    distance_metric,
                    min_depth
                );
                if let Ok(results) = run_physics_sim(&tree, &graph, scalar, max_iters, test_cb) {
                    let mut file_path = PathBuf::new();
                    // file_path.push("triangle_acc_results");
                    file_path.push(out_folder);
                    file_path.push(tree.data().name());
                    match fs::create_dir_all(&file_path) {
                        Ok(_) => {
                            println!("Folder created successfully or already exists.")
                        }
                        Err(e) => eprintln!("Error creating folder: {}", e),
                    }
                    file_path.push(outfile_name);
                    println!("writing to {:?}", file_path.to_str().unwrap());
                    utils::write_results(&file_path, &results)
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
    metric_cb: fn(&mut Vec<(&str, f32)>, &mut Vec<(&str, f32)>) -> f64,
) -> Result<Vec<String>, String> {
    let mut fdg = build_force_directed_graph(&tree, &graph, scalar, max_iters);
    println!("created fdg");
    let mut results: Vec<String> = Vec::with_capacity(max_iters as usize);

    for i in 0..max_iters {
        if i % 100 == 0 {
            println!("time step {}", i);
        }

        fdg.update(&graph, &tree);

        match run_triangle_test(&tree, &graph, &fdg, 5, metric_cb) {
            Ok(accuracy) => {
                results.push(accuracy.to_string());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(results)
}

pub fn run_triangle_test(
    tree: &Treef32,
    clam_graph: &Graphf32,
    fdg: &ForceDirectedGraph,
    num_test_iters: i32,
    metric_cb: fn(&mut Vec<(&str, f32)>, &mut Vec<(&str, f32)>) -> f64,
) -> Result<f64, String> {
    if clam_graph.clusters().len() < 3 {
        return Err("less than 3 clusters in graph".to_string());
    }
    let mut clusters: Vec<_> = clam_graph.clusters().into_iter().map(|c| *c).collect();
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let mut metric_sum: f64 = 0.;

    // let mut results: Vec<f64> =
    //     Vec::with_capacity(clam_graph.clusters().len() * num_test_iters as usize);

    for _ in 0..num_test_iters {
        for a in clam_graph.clusters() {
            clusters.shuffle(&mut rng);
            if let Some(triangle) = utils::choose_two_random_clusters_exclusive(&clusters, a) {
                // let mut unity_a = ClusterDataWrapper::from_cluster(triangle[0]);
                let unity_a = fdg.get_cluster_position(&triangle[0].name())?;
                let unity_b = fdg.get_cluster_position(&triangle[1].name())?;
                let unity_c = fdg.get_cluster_position(&triangle[2].name())?;
                let mut unity_edges = vec![
                    ("ab", unity_a.distance(unity_b)),
                    ("ac", unity_a.distance(unity_c)),
                    ("bc", unity_b.distance(unity_c)),
                ];

                let mut clam_edges = vec![
                    (
                        "ab",
                        triangle[0].distance_to_other(tree.data(), triangle[1]),
                    ),
                    (
                        "ac",
                        triangle[0].distance_to_other(tree.data(), triangle[2]),
                    ),
                    (
                        "bc",
                        triangle[1].distance_to_other(tree.data(), triangle[2]),
                    ),
                ];

                metric_sum += metric_cb(&mut clam_edges, &mut unity_edges);
            }
        }
    }

    let average_distortion =
        metric_sum as f64 / (num_test_iters as f64 * clam_graph.vertex_cardinality() as f64) as f64;

    // results.push(perc_correct);
    return Ok(average_distortion);
    // return Err("shouldn''t reach this".to_string());
}

#[test]
fn run_triangle_equivalency() {
    let (dir, min_cardinality, min_depth, distance_metric, scalar, max_iters, src_folder, target) =
        test_params();

    let outfolder = "edge_equivalence";
    let metric_cb = utils::are_triangles_equivalent;

    run_for_each(
        dir,
        min_cardinality,
        min_depth,
        distance_metric,
        scalar,
        max_iters,
        &src_folder,
        outfolder,
        metric_cb,
    );
}

#[test]
fn run_edge_distortion() {
    let (dir, min_cardinality, min_depth, distance_metric, scalar, max_iters, src_folder, target) =
        test_params();

    let outfolder = "edge_distortion";
    let metric_cb = utils::calc_triangle_distortion;

    run_for_each(
        dir,
        min_cardinality,
        min_depth,
        distance_metric,
        scalar,
        max_iters,
        &src_folder,
        outfolder,
        metric_cb,
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
    metric_cb: fn(&mut Vec<(&str, f32)>, &mut Vec<(&str, f32)>) -> f64,
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
                if let Some(filename) = parts.first() {
                    if !finished.contains(&filename.to_string()) {
                        println!("filename: {}", filename);
                        // if *filename == "smtp" {
                        run_test_on_file(
                            filename,
                            &src_folder,
                            outfolder,
                            distance_metric,
                            min_cardinality,
                            min_depth,
                            max_iters,
                            scalar,
                            metric_cb,
                        );
                        // }
                        finished.insert(filename.to_string());
                    }
                }
            }
        }
    }
}
