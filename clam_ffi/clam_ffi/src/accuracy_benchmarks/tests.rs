use std::{
    collections::{HashMap, HashSet},
    env,
    fs::{self, ReadDir},
    io::Read,
    path::{Path, PathBuf},
};

use abd_clam::{
    graph::{self, Graph},
    Dataset, PartitionCriteria, Tree,
};
use distances::Number;
use rand::{seq::SliceRandom, thread_rng};

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
        Some("wine".to_string()),
        // single_target,
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
    test_cb: fn(&mut [(&str, f32); 3], &mut [(&str, f32); 3]) -> f64,
) {
    match Handle::create_dataset(filename, &src_folder, distance_metric, false) {
        Ok(data) => {
            println!("created dataset");
            let criteria = PartitionCriteria::new(true).with_min_cardinality(min_cardinality);

            let tree = Tree::new(data, Some(1)).partition(&criteria, None);

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
                    min_depth,
                );
                let descriptor_file = format!(
                    "{}_{}_{:?}_{}.txt",
                    tree.data().name(),
                    min_cardinality.to_string(),
                    distance_metric,
                    min_depth,
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
                    utils::write_results(&file_path, &results);

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
                } else {
                    panic!("collecting data for this graph failed");
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
    metric_cb: fn(&mut [(&str, f32); 3], &mut [(&str, f32); 3]) -> f64,
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
    metric_cb: fn(&mut [(&str, f32); 3], &mut [(&str, f32); 3]) -> f64,
) -> Result<f64, String> {
    if clam_graph.ordered_clusters().len() < 3 {
        return Err("less than 3 clusters in graph".to_string());
    }
    let mut clusters: Vec<_> = clam_graph
        .ordered_clusters()
        .into_iter()
        .map(|c| *c)
        .collect();
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let mut metric_sum: f64 = 0.;

    // let mut results: Vec<f64> =
    //     Vec::with_capacity(clam_graph.clusters().len() * num_test_iters as usize);
    let mut valid_count = 0;
    for _ in 0..num_test_iters {
        for a in clam_graph.ordered_clusters() {
            // clusters.shuffle(&mut rng);
            clusters.partial_shuffle(&mut rng, 5);
            if let Some(chosen_clusters) = utils::choose_two_random_clusters_exclusive(&clusters, a)
            {
                if let Ok(mut clam_edges) = utils::triangle_from_clusters(tree, &chosen_clusters) {
                    if let Ok(mut unity_edges) = utils::get_unity_triangle(&chosen_clusters, &fdg) {
                        metric_sum += metric_cb(&mut clam_edges, &mut unity_edges);
                        valid_count += 1;
                    }
                }
            }
        }
    }
    if valid_count == 0 {
        return Err("no valid triangles found".to_string());
    }
    let average_distortion = metric_sum as f64 / (valid_count as f64) as f64;

    // results.push(perc_correct);
    return Ok(average_distortion);
    // return Err("shouldn''t reach this".to_string());
}

#[test]
fn edge_equivalence() {
    for i in 4..15 {
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
        ) = test_params(None);
        // ) = test_params(None);

        // let outfolder = "edge_equivalence";
        let mut out_folder = PathBuf::new();
        out_folder.push(out_folder_root);
        out_folder.push("edge_equivalence");
        let metric_cb = utils::are_triangles_equivalent;

        run_for_each(
            dir,
            min_cardinality,
            i,
            distance_metric,
            scalar,
            max_iters,
            &src_folder,
            out_folder.to_str().unwrap(),
            target,
            metric_cb,
        );
    }
}

#[test]
fn edge_distortion() {
    for i in 4..12 {
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
        ) = test_params(None);

        // ) = test_params(Some("vertebral".to_string()));
        // test_params(None);

        let mut out_folder = PathBuf::new();
        out_folder.push(out_folder_root);
        out_folder.push("edge_distortion");
        let metric_cb = utils::calc_edge_distortion;

        run_for_each(
            dir,
            min_cardinality,
            i,
            distance_metric,
            scalar,
            max_iters,
            &src_folder,
            out_folder.to_str().unwrap(),
            target,
            metric_cb,
        );
    }
}

#[test]
fn angle_distortion() {
    for i in 4..12 {
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
        ) = test_params(None);

        // let outfolder = "angle_distortion";
        let mut out_folder = PathBuf::new();
        out_folder.push(out_folder_root);
        out_folder.push("angle_distortion");
        let metric_cb = utils::calc_angle_distortion;

        run_for_each(
            dir,
            min_cardinality,
            i,
            distance_metric,
            scalar,
            max_iters,
            &src_folder,
            out_folder.to_str().unwrap(),
            target,
            metric_cb,
        );
    }
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
    metric_cb: fn(&mut [(&str, f32); 3], &mut [(&str, f32); 3]) -> f64,
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
                        if let Some(target) = &single_target {
                            if target == *filename {
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
                                break;
                            }
                        } else {
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
                        }

                        // }
                        finished.insert(filename.to_string());
                    }
                }
            }
        }
    }
}

fn run_umap_test_on_file(
    file_path: &str,
    tree: &Treef32,
    metric_cb: fn(&mut [(&str, f32); 3], &mut [(&str, f32); 3]) -> f64,
) -> Result<f64, String> {
    let mut valid_count = 0;
    let mut metric_sum: f64 = 0.;

    if let Ok(positions) = utils::read_umap_positions(file_path) {
        let range_start: usize = 0;
        let range_end: usize = positions.len() - 1;
        let mut range: Vec<usize> = (range_start..=range_end).collect();

        if let Some(max_value) = range.iter().max() {
            println!("Highest value: {}", max_value);
        } else {
            println!("The vector is empty.");
        }

        let data = tree.data();
        if let Some(max_value) = data.permuted_indices().unwrap().iter().max() {
            println!("Highest value perm: {}", max_value);
        } else {
            println!("The vector is empty.");
        }

        let mut rng = thread_rng();

        for _ in 0..range_end * 3 {
            let permuted_indices = utils::randomly_select_three_indices(&mut range, &mut rng);
            // println!("test1");
            let original_indices = (
                data.original_index(permuted_indices.0 as usize),
                data.original_index(permuted_indices.1 as usize),
                data.original_index(permuted_indices.2 as usize),
            );
            // println!("test2");

            let selected_positions = [
                positions[original_indices.0],
                positions[original_indices.1],
                positions[original_indices.2],
            ];

            if let Ok(mut umap_triangle) = utils::positions_to_distances(&selected_positions) {
                let clusters = [
                    tree.get_cluster(permuted_indices.0, 1).unwrap(),
                    tree.get_cluster(permuted_indices.1, 1).unwrap(),
                    tree.get_cluster(permuted_indices.2, 1).unwrap(),
                ];

                if let Ok(mut clam_triangle) = utils::triangle_from_clusters(tree, &clusters) {
                    // let mut results: Vec<f64> =
                    //     Vec::with_capacity(clam_graph.clusters().len() * num_test_iters as usize);
                    metric_sum += metric_cb(&mut clam_triangle, &mut umap_triangle);
                    valid_count += 1;
                }
            }

            // results.push(perc_correct);
        }
    }
    if valid_count == 0 {
        return Err("no valid triangles found".to_string());
    }
    let average_distortion = metric_sum as f64 / (valid_count as f64) as f64;
    return Ok(average_distortion);
}

#[test]
fn umap_test_edge_e9quivalence_umap() {
    let (
        search_dir,
        _min_cardinality,
        _min_depth,
        distance_metric,
        _scalar,
        _max_iters,
        data_folder,
        out_folder_root,
        target,
        // ) = test_params(None);
    ) = test_params(Some("satellite".to_string()));

    let mut out_folder = PathBuf::new();
    out_folder.push(out_folder_root);
    out_folder.push("umap_edge_equivalence");
    let metric_cb = utils::are_triangles_equivalent;

    run_for_each_umap(
        search_dir,
        target.unwrap().as_str(),
        &data_folder,
        out_folder.to_str().unwrap(),
        distance_metric,
        metric_cb,
    );
}

#[test]
fn umap_test_edge_1distortion() {
    let (
        search_dir,
        _min_cardinality,
        _min_depth,
        distance_metric,
        _scalar,
        _max_iters,
        data_folder,
        out_folder_root,
        target,
        // ) = test_params(None);
    ) = test_params(None);

    let mut out_folder = PathBuf::new();
    out_folder.push(out_folder_root);
    out_folder.push("umap_edge_distortion");
    let metric_cb = utils::calc_edge_distortion;

    run_for_each_umap(
        search_dir,
        target.unwrap().as_str(),
        &data_folder,
        out_folder.to_str().unwrap(),
        distance_metric,
        metric_cb,
    );
}

#[test]
fn umap_test_ang1le_distortion() {
    let (
        search_dir,
        _min_cardinality,
        _min_depth,
        distance_metric,
        _scalar,
        _max_iters,
        data_folder,
        out_folder_root,
        target,
        // ) = test_params(None);
    ) = test_params(None);

    let mut out_folder = PathBuf::new();
    out_folder.push(out_folder_root);
    out_folder.push("umap_angle_distortion");
    let metric_cb = utils::calc_angle_distortion;

    run_for_each_umap(
        search_dir,
        target.unwrap().as_str(),
        &data_folder,
        out_folder.to_str().unwrap(),
        distance_metric,
        metric_cb,
    );
}

fn run_for_each_umap(
    dir: ReadDir,
    data_name: &str,
    src_folder: &PathBuf,
    outfolder: &str,
    distance_metric: DistanceMetric,
    metric_cb: fn(&mut [(&str, f32); 3], &mut [(&str, f32); 3]) -> f64,
) {
    match Handle::create_dataset(data_name, &src_folder, distance_metric, false) {
        Ok(data) => {
            println!("created dataset {}", data_name);
            let criteria = PartitionCriteria::new(true).with_min_cardinality(1);

            let tree = Tree::new(data, Some(1)).partition(&criteria, None);
            println!("tree card :{}", tree.cardinality());
            println!("tree data name :{}", tree.data().name());
            // let dir_path = ;
            // let dir_path = "../../umap/".to_string() + data_name;

            let mut scores: HashMap<u32, Vec<f64>> = HashMap::new();
            for _ in 0..5 {
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
                                    println!("positions file : {}", positions_file);

                                    let avg =
                                        run_umap_test_on_file(positions_file, &tree, metric_cb)
                                            .unwrap();
                                    println!("avg: {}", avg);
                                    let k = utils::extract_umap_k(positions_file).unwrap();
                                    if !scores.contains_key(&k) {
                                        scores.insert(k, Vec::new());
                                    }
                                    scores.get_mut(&k).unwrap().push(avg);
                                }
                                // Process the file
                                println!("Found file: {}", entry_path.display());
                            }
                        }
                    }
                } else {
                    // eprintln!("Failed to read directory {}", dir_path);
                }
            }

            println!("scores: {:?}", scores);
            let mut outpath = PathBuf::new();
            outpath.push(outfolder);
            fs::create_dir_all(&outpath);

            outpath.push(data_name.to_string() + ".csv");
            // outpath.push(".csv");
            println!("ouptath {:?}", outpath);
            let mut avg_scores: HashMap<u32, f64> = HashMap::new();
            for (key, value) in scores {
                let avg: f64 = value.iter().sum::<f64>() / value.len().as_f64();
                avg_scores.insert(key, avg);
            }
            utils::write_umap_scores_to_file(outpath.to_str().unwrap(), &avg_scores);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
