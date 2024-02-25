use std::{
    cmp::Ordering,
    collections::HashSet,
    error::Error,
    fs::OpenOptions,
    io::{self, Write},
    ops::Add,
};

use abd_clam::{Cluster, Dataset, Edge};
use distances::Number;
use rand::seq::IteratorRandom;

use crate::{
    debug,
    ffi_impl::cluster_data_wrapper::ClusterDataWrapper,
    utils::{error::FFIError, types::InHandlePtr},
    CBFnNodeVisitorMut,
};

fn choose_two_random_clusters_exclusive<'a, U: Number>(
    clusters: &HashSet<&'a Cluster<U>>,
    cluster: &'a Cluster<U>,
    mut rng: &mut rand::prelude::ThreadRng,
) -> Option<(&'a Cluster<U>, &'a Cluster<U>)> {
    if let Some(cluster1) = clusters.iter().choose(&mut rng) {
        if let Some(cluster2) = clusters.iter().choose(&mut rng) {
            if &cluster != cluster1 && &cluster != cluster2 && cluster1 != cluster2 {
                return Some((cluster1, cluster2));
            }
        }
    }
    return None;
}

pub fn run_triangle_test_impl(
    context: InHandlePtr,
    location_getter: CBFnNodeVisitorMut,
) -> FFIError {
    if let Some(handle) = context {
        // handle.physics_update_async(location_getter)
        if let Some(tree) = handle.tree() {
            if let Some(data) = handle.data() {
                if let Some(clam_graph) = handle.clam_graph() {
                    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
                    let mut total_count = 0;
                    let mut correct_triangle_count = 0;
                    let clusters: Vec<_> = clam_graph.clusters().into_iter().collect();
                    for a in clam_graph.clusters() {
                        let mut correct_edge_count = 0;
                        if let Some((b, c)) =
                            choose_two_random_clusters_exclusive(clam_graph.clusters(), a, &mut rng)
                        {
                            total_count += 1;

                            let ab = Edge::new(a, b, a.distance_to_other(data, b));
                            let ac = Edge::new(a, c, a.distance_to_other(data, c));
                            let bc = Edge::new(b, c, b.distance_to_other(data, c));

                            let mut unity_a = ClusterDataWrapper::from_cluster(a);
                            let mut unity_b = ClusterDataWrapper::from_cluster(b);
                            let mut unity_c = ClusterDataWrapper::from_cluster(c);

                            location_getter(Some(unity_a.data_mut()));
                            location_getter(Some(unity_b.data_mut()));
                            location_getter(Some(unity_c.data_mut()));

                            let unity_ab =
                                Edge::new(a, b, unity_a.data().pos.distance(unity_b.data().pos));
                            let unity_ac =
                                Edge::new(a, c, unity_a.data().pos.distance(unity_c.data().pos));
                            let unity_bc =
                                Edge::new(b, c, unity_b.data().pos.distance(unity_c.data().pos));

                            let mut clam_edges = vec![("ab", ab), ("ac", ac), ("bc", bc)];
                            let mut unity_edges =
                                vec![("ab", unity_ab), ("ac", unity_ac), ("bc", unity_bc)];

                            // let clam_string = format!(
                            //     "[{}: {}], [{}: {}], [{}: {}]",
                            //     clam_edges[0].0,
                            //     clam_edges[0].1.distance(),
                            //     clam_edges[1].0,
                            //     clam_edges[1].1.distance(),
                            //     clam_edges[2].0,
                            //     clam_edges[2].1.distance()
                            // );

                            // let unity_string = format!(
                            //     "[{}: {}], [{}: {}], [{}: {}]",
                            //     unity_edges[0].0,
                            //     unity_edges[0].1.distance(),
                            //     unity_edges[1].0,
                            //     unity_edges[1].1.distance(),
                            //     unity_edges[2].0,
                            //     unity_edges[2].1.distance()
                            // );
                            // debug!("clam: {}", clam_string);
                            // debug!("unity: {}", unity_string);
                            clam_edges.sort_by(|a, b| {
                                a.1.distance().partial_cmp(&b.1.distance()).unwrap()
                            });

                            unity_edges.sort_by(|a, b| {
                                a.1.distance().partial_cmp(&b.1.distance()).unwrap()
                            });

                            // debug!("after sort:");

                            // let clam_string = format!(
                            //     "[{}: {}], [{}: {}], [{}: {}]",
                            //     clam_edges[0].0,
                            //     clam_edges[0].1.distance(),
                            //     clam_edges[1].0,
                            //     clam_edges[1].1.distance(),
                            //     clam_edges[2].0,
                            //     clam_edges[2].1.distance()
                            // );

                            // let unity_string = format!(
                            //     "[{}: {}], [{}: {}], [{}: {}]",
                            //     unity_edges[0].0,
                            //     unity_edges[0].1.distance(),
                            //     unity_edges[1].0,
                            //     unity_edges[1].1.distance(),
                            //     unity_edges[2].0,
                            //     unity_edges[2].1.distance()
                            // );
                            // debug!("clam: {}", clam_string);
                            // debug!("unity: {}", unity_string);

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

                    let output = format!(
                        "{}/{} = {}. Total Attempts = {}\n",
                        correct_triangle_count.to_string(),
                        total_count,
                        String::from(
                            (correct_triangle_count.as_f64() / total_count.as_f64()).to_string()
                        ),
                        clam_graph.clusters().len().to_string()
                    );

                    let fname = format!("{}/{}", "triangle_test_results", tree.data().name());
                    append_to_file(fname.as_str(), &output);
                }
            }
        }
    } else {
        return FFIError::NullPointerPassed;
    }

    return FFIError::LoadTreeFailed;
}

fn append_to_file(filename: &str, content: &str) -> Result<(), io::Error> {
    // Open the file in append mode or create it if it doesn't exist
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;

    // Write the content to the file
    file.write_all(content.as_bytes())?;

    // Ensure all data is written to disk
    file.sync_all()?;

    Ok(())
}
