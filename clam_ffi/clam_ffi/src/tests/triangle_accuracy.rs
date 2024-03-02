use core::panic;
use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    env,
    fs::{self, OpenOptions, ReadDir},
    path::{Path, PathBuf},
    str::FromStr,
};

use csv;

use abd_clam::{Dataset, Graph, PartitionCriteria, Tree, VecDataset};
use csv::WriterBuilder;
use distances::Number;
use nalgebra::max;
use std::fs::File;

use crate::{
    graph::{
        force_directed_graph::ForceDirectedGraph,
        graph_builder::{self, build_force_directed_graph},
    },
    handle::handle::Handle,
    utils::{
        anomaly_readers,
        distances::DistanceMetric,
        error::FFIError,
        scoring_functions::{enum_to_function, ScoringFunction},
        types::{DataSetf32, Graphf32, Treef32},
    },
};

use super::test;

fn cb<T: Number>(cb: fn(&mut Vec<(&str, f32)>, &mut Vec<(&str, f32)>) -> T) {}

fn edge_distortion() {
    let (
        dir,
        min_cardinality,
        min_depth,
        distance_metric,
        scalar,
        max_iters,
        src_folder,
        target_file,
    ) = test_params();
    let mut finished: HashSet<String> = HashSet::new();

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
                        if let Some(target_file) = &target_file {
                            if *filename == target_file {
                                run_test_on_file(
                                    filename,
                                    &src_folder,
                                    distance_metric,
                                    min_cardinality,
                                    min_depth,
                                    max_iters,
                                    scalar,
                                );
                            }
                        }
                        finished.insert(filename.to_string());
                    }
                }
            }
        }
    }
}

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
    let dir = fs::read_dir(dir_path).unwrap();
    let min_cardinality = 50;
    let min_depth = 8;
    let distance_metric = DistanceMetric::Euclidean;
    let scalar = 100.0;
    let max_iters = 1000;
    let src_folder = PathBuf::from(dir_path);

    (
        dir,
        min_cardinality,
        min_depth,
        distance_metric,
        scalar,
        max_iters,
        src_folder,
        Some("smtp".to_string()),
    )
}

#[test]
fn triangle_accuracy<'a>() {
    if let Ok(current_dir) = env::current_dir() {
        // If successful, print the current working directory
        println!("Current working directory: {}", current_dir.display());
    } else {
        // If there was an error getting the current directory, print an error message
        eprintln!("Failed to get the current working directory");
    }
    // Specify the directory path you want to iterate through
    // let dir_path = "../../data/anomaly_data/preprocessed";
    let dir_path = Path::new("../../data/anomaly_data/preprocessed");

    // Open the directory
    let dir = fs::read_dir(dir_path).unwrap();
    let min_cardinality = 50;
    let min_depth = 8;
    let distance_metric = DistanceMetric::Euclidean;
    let scalar = 100.0;
    let max_iters = 1000;
    let src_folder = PathBuf::from(dir_path);
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
                        if *filename == "smtp" {
                            run_test_on_file(
                                filename,
                                &src_folder,
                                distance_metric,
                                min_cardinality,
                                min_depth,
                                max_iters,
                                scalar,
                            );
                        }
                        finished.insert(filename.to_string());
                    }
                }
            }
        }
    }
}

fn run_physics_sim(
    tree: &Treef32,
    graph: &Graphf32,
    scalar: f32,
    max_iters: i32,
) -> Result<Vec<String>, String> {
    let mut fdg = build_force_directed_graph(&tree, &graph, scalar, max_iters);
    println!("created fdg");
    let mut results: Vec<String> = Vec::with_capacity(max_iters as usize);

    for i in 0..max_iters {
        if i % 100 == 0 {
            println!("time step {}", i);
        }

        fdg.update(&graph, &tree);
        // Determine the value of last_run based on whether i equals max_iters
        match test::run_triangle_test(&tree, &graph, &fdg, 5) {
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

fn write_results(outpath: &PathBuf, results: &Vec<String>) {
   

    let file = OpenOptions::new().create(true).append(true).open(outpath);
    if let Ok(file) = file {
        // Create a CSV writer
        let mut writer = WriterBuilder::new()
            .delimiter(b',') // Set delimiter (optional)
            .from_writer(file);

        // Write the data to the CSV file
        let _ = writer.write_record(results);

        // Flush and close the writer to ensure all data is written
        let _ = writer.flush();
    }
}

fn run_test_on_file(
    filename: &str,
    src_folder: &PathBuf,
    distance_metric: DistanceMetric,
    min_cardinality: usize,
    min_depth: usize,
    max_iters: i32,
    scalar: f32,
) {
    match Handle::create_dataset(filename, &src_folder, distance_metric, false) {
        Ok(data) => {
            println!("created dataset");
            let criteria = PartitionCriteria::new(true).with_min_cardinality(min_cardinality);

            let tree = Tree::new(data, Some(1))
                .partition(&criteria)
                .with_ratios(false);

            // let min_depth = 6;
            // let min_depth = tree.depth().min(4);

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
                if let Ok(results) = run_physics_sim(&tree, &graph, scalar, max_iters) {
                    let mut file_path = PathBuf::new();
                    // file_path.push("triangle_acc_results");
                    file_path.push("triangle_acc_results");
                    file_path.push(tree.data().name());
                    match fs::create_dir_all(&file_path) {
                        Ok(_) => {
                            println!("Folder created successfully or already exists.")
                        }
                        Err(e) => eprintln!("Error creating folder: {}", e),
                    }
                    file_path.push(outfile_name);
                    println!("writing to {:?}", file_path.to_str().unwrap());
                    write_results(&file_path, &results)
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}

fn run_edge_distortion(
    filename: &str,
    src_folder: &PathBuf,
    distance_metric: DistanceMetric,
    min_cardinality: usize,
    min_depth: usize,
    max_iters: i32,
    scalar: f32,
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
                if let Ok(results) = run_physics_sim(&tree, &graph, scalar, max_iters) {
                    let mut file_path = PathBuf::new();
                    // file_path.push("triangle_acc_results");
                    file_path.push("triangle_distortion_results");
                    file_path.push(tree.data().name());
                    match fs::create_dir_all(&file_path) {
                        Ok(_) => {
                            println!("Folder created successfully or already exists.")
                        }
                        Err(e) => eprintln!("Error creating folder: {}", e),
                    }
                    file_path.push(outfile_name);
                    println!("writing to {:?}", file_path.to_str().unwrap());
                    write_results(&file_path, &results)
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
