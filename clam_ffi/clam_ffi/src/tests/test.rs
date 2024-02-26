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
    utils::{
        self,
        error::FFIError,
        types::{Clusterf32, InHandlePtr},
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
                            let mut correct_edge_count = 0;
                            if let Some(triangle) =
                                choose_two_random_clusters_exclusive(&clusters, a)
                            {
                                let mut unity_a = ClusterDataWrapper::from_cluster(triangle[0]);
                                let mut unity_b = ClusterDataWrapper::from_cluster(triangle[1]);
                                let mut unity_c = ClusterDataWrapper::from_cluster(triangle[2]);

                                location_getter(Some(unity_a.data_mut()));
                                location_getter(Some(unity_b.data_mut()));
                                location_getter(Some(unity_c.data_mut()));

                                let mut clam_edges = vec![
                                    ("ab", triangle[0].distance_to_other(data, triangle[1])),
                                    ("ac", triangle[0].distance_to_other(data, triangle[2])),
                                    ("bc", triangle[1].distance_to_other(data, triangle[2])),
                                ];
                                let mut unity_edges = vec![
                                    ("ab", unity_a.data().pos.distance(unity_b.data().pos)),
                                    ("ac", unity_a.data().pos.distance(unity_c.data().pos)),
                                    ("bc", unity_b.data().pos.distance(unity_c.data().pos)),
                                ];

                                clam_edges.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                                unity_edges.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                                for (e1, e2) in clam_edges.iter().zip(unity_edges.iter()) {
                                    if e1.0 == e2.0 {
                                        correct_edge_count += 1;
                                    }
                                }

                                if correct_edge_count == 3 {
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
