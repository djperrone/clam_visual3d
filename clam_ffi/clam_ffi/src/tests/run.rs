use std::fs;

use abd_clam::{Graph, PartitionCriteria, Tree, VecDataset};

use crate::{
    graph::{
        force_directed_graph::ForceDirectedGraph,
        graph_builder::{self, build_force_directed_graph_no_handle},
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

fn create_handle(filename: &str, min_cardinality: usize, is_expensive: bool) {}

fn main<'a>() -> Result<Handle<'a>, String> {
    // Specify the directory path you want to iterate through
    let dir_path = "../data/anomaly_data/preprocessed";

    // Open the directory
    let dir = fs::read_dir(dir_path).unwrap();
    let min_cardinality = 1;
    let distance_metric = DistanceMetric::Euclidean;
    let scalar = 100.0;
    let max_iters = 2000;

    // Iterate through each entry in the directory
    for filename in dir {
        if let Ok(filename) = filename {
            // Convert the file name to a string
            if let Some(filename) = filename.file_name().to_str() {
                if let Ok(data) = Handle::create_dataset(filename, distance_metric, false) {
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
                        let fdg =
                            build_force_directed_graph_no_handle(&tree, &graph, scalar, max_iters);
                    }
                }

                //     if let Ok(mut handle) = Handle::new(
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
    Err("failed".to_string())
}
