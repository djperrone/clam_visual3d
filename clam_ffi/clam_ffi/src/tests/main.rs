use core::panic;
use std::{
    env, fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use abd_clam::{Dataset, Graph, PartitionCriteria, Tree, VecDataset};

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
        types::DataSetf32,
    },
};

use super::test;

fn create_handle(filename: &str, min_cardinality: usize, is_expensive: bool) {}
#[test]
fn main<'a>() {
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

    // Use the path as needed
    println!("Directory path1234: {:?}", dir_path);
    // panic!();
    // Open the directory
    let dir = fs::read_dir(dir_path).unwrap();
    let min_cardinality = 1;
    let distance_metric = DistanceMetric::Euclidean;
    let scalar = 100.0;
    let max_iters = 2000;
    let src_folder = PathBuf::from(dir_path);
    println!("src folder {}", src_folder.to_str().unwrap());
    // Iterate through each entry in the directory
    for filename in dir {
        if let Ok(filename) = filename {
            // Convert the file name to a string
            if let Some(filename) = filename.file_name().to_str() {
                let parts: Vec<&str> = filename.split('_').collect();

                // Check if the split operation produced at least one part
                if let Some(filename) = parts.first() {
                    println!("filename: {}", filename);

                    match Handle::create_dataset(filename, &src_folder, distance_metric, false) {
                        Ok(data) => {
                            println!("created dataset");
                            let criteria =
                                PartitionCriteria::new(true).with_min_cardinality(min_cardinality);

                            let tree = Tree::new(data, Some(1))
                                .partition(&criteria)
                                .with_ratios(false);

                            let min_depth = tree.depth() / 2;

                            if let Ok(graph) = Graph::from_tree(
                                &tree,
                                &enum_to_function(&ScoringFunction::LrEuclideanCc).unwrap(),
                                min_depth,
                            ) {
                                println!("created graph");

                                let mut fdg =
                                    build_force_directed_graph(&tree, &graph, scalar, max_iters);
                                println!("created fdg");

                                for i in 0..max_iters {
                                    println!("time step {}", i);

                                    fdg.update(&graph, &tree);
                                    // Determine the value of last_run based on whether i equals max_iters
                                    let last_run = i == max_iters - 1;
                                    test::run_triangle_test_impl_no_handle(
                                        &tree,
                                        &graph,
                                        &fdg,
                                        3,
                                        last_run,
                                        tree.data().name(),
                                    );
                                }
                                println!("rand fdg sim");
                            }
                        }
                        Err(e) => {
                            println!("{:?}", e);
                        } //     if let Ok(mut handle) = Handle::new(
                          //         String::from(filename).as_str(),
                          //         min_cardinality,
                          //         distance_metric,
                          //         false,
                          //     ) {

                          //         {
                          //             let graph_build_error = {
                          //                 handle.init_clam_graph_no_visual(
                          //                     ScoringFunction::LrEuclideanCc,
                          //                     min_depth,
                          //                 )
                          //             };
                          //         }

                          // match graph_builder::build_force_directed_graph(&handle, scalar, max_iters)
                          //         {
                          //             Ok(g) => {
                          //                 // handle.set_graph(g);
                          //             }
                          //             Err(e) => {}
                          //         }
                          //     }
                    }
                }
            }
        }
    }
    // Err("failed".to_string())
}
