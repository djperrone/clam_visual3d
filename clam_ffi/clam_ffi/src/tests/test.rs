use std::{
    cmp::Ordering,
    collections::HashSet,
    error::Error,
    ffi::{c_char, CStr},
    fs::OpenOptions,
    io::{self, Write},
    ops::Add,
};

use abd_clam::{Cluster, Dataset, Edge};
use distances::Number;
use rand::seq::{IteratorRandom, SliceRandom};

use crate::{
    debug,
    ffi_impl::cluster_data_wrapper::ClusterDataWrapper,
    graph::force_directed_graph::ForceDirectedGraph,
    utils::{
        self,
        error::FFIError,
        types::{Clusterf32, Graphf32, InHandlePtr, Treef32},
    },
    CBFnNodeVisitorMut,
};

fn choose_two_random_clusters_exclusive<'a, U: Number>(
    clusters: &Vec<&'a Cluster<U>>,
    cluster: &'a Cluster<U>,
) -> Option<Vec<&'a Cluster<U>>> {
    let mut triangle: Vec<&'a Cluster<U>> = Vec::new();
    triangle.push(cluster);
    for c in clusters {
        if triangle.len() < 3 {
            if c != &cluster {
                triangle.push(c);
            }
        } else {
            return Some(triangle);
        }
    }
    // if let Some(cluster1) = clusters.iter().choose(&mut rng) {
    //     if let Some(cluster2) = clusters.iter().choose(&mut rng) {
    //         if &cluster != cluster1 && &cluster != cluster2 && cluster1 != cluster2 {
    //             return Some((cluster1, cluster2));
    //         }
    //     }
    // }
    return None;
}

fn are_triangles_equivalent(
    clam_edges: &mut Vec<(&str, f32)>,
    unity_edges: &mut Vec<(&str, f32)>,
) -> bool {
    clam_edges.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    unity_edges.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let mut correct_edge_count = 0;
    for (e1, e2) in clam_edges.iter().zip(unity_edges.iter()) {
        if e1.0 == e2.0 {
            correct_edge_count += 1;
        }
    }

    if correct_edge_count == 3 {
        return true;
    }
    return false;
}

fn get_unity_triangle<'a>(
    a: &Clusterf32,
    b: &Clusterf32,
    c: &Clusterf32,
    location_getter: CBFnNodeVisitorMut,
) -> Vec<(&'a str, f32)> {
    let mut unity_a = ClusterDataWrapper::from_cluster(a);
    let mut unity_b = ClusterDataWrapper::from_cluster(b);
    let mut unity_c = ClusterDataWrapper::from_cluster(c);

    location_getter(Some(unity_a.data_mut()));
    location_getter(Some(unity_b.data_mut()));
    location_getter(Some(unity_c.data_mut()));

    vec![
        ("ab", unity_a.data().pos.distance(unity_b.data().pos)),
        ("ac", unity_a.data().pos.distance(unity_c.data().pos)),
        ("bc", unity_b.data().pos.distance(unity_c.data().pos)),
    ]
}
pub fn run_triangle_test_impl(
    context: InHandlePtr,
    num_test_iters: i32,
    last_run: bool,
    out_path: *const c_char,
    location_getter: CBFnNodeVisitorMut,
) -> FFIError {
    if let Some(handle) = context {
        // handle.physics_update_async(location_getter)
        if let Some(tree) = handle.tree() {
            if let Some(data) = handle.data() {
                if let Some(clam_graph) = handle.clam_graph() {
                    if clam_graph.clusters().len() < 3 {
                        return FFIError::GraphBuildFailed;
                    }
                    let mut clusters: Vec<_> =
                        clam_graph.clusters().into_iter().map(|c| *c).collect();
                    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
                    let mut correct_triangle_count = 0;

                    for _ in 0..num_test_iters {
                        for a in clam_graph.clusters() {
                            clusters.shuffle(&mut rng);
                            if let Some(triangle) =
                                choose_two_random_clusters_exclusive(&clusters, a)
                            {
                                let mut clam_edges = vec![
                                    ("ab", triangle[0].distance_to_other(data, triangle[1])),
                                    ("ac", triangle[0].distance_to_other(data, triangle[2])),
                                    ("bc", triangle[1].distance_to_other(data, triangle[2])),
                                ];

                                let mut unity_edges = get_unity_triangle(
                                    triangle[0],
                                    triangle[1],
                                    triangle[2],
                                    location_getter,
                                );

                                if are_triangles_equivalent(&mut clam_edges, &mut unity_edges) {
                                    correct_triangle_count += 1;
                                }
                            }
                        }
                    }

                    let perc_correct = correct_triangle_count as f64
                        / (num_test_iters as f64 * clam_graph.vertex_cardinality() as f64) as f64;

                    let output = if last_run {
                        format!("{}\n", perc_correct)
                    } else {
                        format!("{},", perc_correct)
                    };

                    // let fname = format!("{}/{}.csv", "triangle_test_results", tree.data().name());
                    let fname = unsafe { CStr::from_ptr(out_path) };
                    if let Ok(fname) = fname.to_str() {
                        utils::helpers::append_to_file(fname, &output);
                    }
                }
            }
        }
    } else {
        return FFIError::NullPointerPassed;
    }

    return FFIError::LoadTreeFailed;
}

pub fn run_triangle_test_impl_no_handle(
    tree: &Treef32,
    clam_graph: &Graphf32,
    fdg: &ForceDirectedGraph,
    num_test_iters: i32,
    last_run: bool,
    out_path: &str,
) -> Result<(), String> {
    if clam_graph.clusters().len() < 3 {
        return Err("less than 3 clusters in graph".to_string());
    }
    let mut clusters: Vec<_> = clam_graph.clusters().into_iter().map(|c| *c).collect();
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let mut correct_triangle_count = 0;

    for _ in 0..num_test_iters {
        for a in clam_graph.clusters() {
            clusters.shuffle(&mut rng);
            if let Some(triangle) = choose_two_random_clusters_exclusive(&clusters, a) {
                // let mut unity_a = ClusterDataWrapper::from_cluster(triangle[0]);
                let unity_a = fdg.get_cluster_position(&triangle[0].name())?;
                let unity_b = fdg.get_cluster_position(&triangle[1].name())?;
                let unity_c = fdg.get_cluster_position(&triangle[2].name())?;
                let mut unity_edges = vec![
                    ("ab", unity_a.distance(unity_b)),
                    ("ac", unity_a.distance(unity_c)),
                    ("bc", unity_b.distance(unity_c)),
                ];

                // let mut unity_b = ClusterDataWrapper::from_cluster(triangle[1]);
                // let mut unity_c = ClusterDataWrapper::from_cluster(triangle[2]);

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

                if are_triangles_equivalent(&mut clam_edges, &mut unity_edges) {
                    correct_triangle_count += 1;
                }
            }
        }

        let perc_correct = correct_triangle_count as f64
            / (num_test_iters as f64 * clam_graph.vertex_cardinality() as f64) as f64;

        let output = if last_run {
            format!("{}\n", perc_correct)
        } else {
            format!("{},", perc_correct)
        };

        // let fname = format!("{}/{}.csv", "triangle_test_results", tree.data().name());
        //  let fname = unsafe { CStr::from_ptr(out_path) };
        // if let Ok(fname) = fname.to_str() {
        utils::helpers::append_to_file(out_path, &output);
        // }
    }

    return Ok(());
}
