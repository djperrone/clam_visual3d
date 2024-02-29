use std::{fs::OpenOptions, path::PathBuf};

use abd_clam::Cluster;
use csv::WriterBuilder;
use distances::Number;

use crate::{
    ffi_impl::cluster_data_wrapper::ClusterDataWrapper, utils::types::Clusterf32,
    CBFnNodeVisitorMut,
};

pub fn choose_two_random_clusters_exclusive<'a, U: Number>(
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
    return None;
}

pub fn are_triangles_equivalent(
    clam_edges: &mut Vec<(&str, f32)>,
    unity_edges: &mut Vec<(&str, f32)>,
) -> f64 {
    clam_edges.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    unity_edges.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let mut correct_edge_count = 0;
    for (e1, e2) in clam_edges.iter().zip(unity_edges.iter()) {
        if e1.0 == e2.0 {
            correct_edge_count += 1;
        }
    }

    if correct_edge_count == 3 {
        return 1.0;
    }
    return 0.0;
}

pub fn calc_triangle_distortion(
    clam_edges: &mut Vec<(&str, f32)>,
    unity_edges: &mut Vec<(&str, f32)>,
) -> f64 {
    // clam_edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    // unity_edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let perimeter_ref: f32 = clam_edges.iter().map(|&(_, value)| value).sum();
    let perimeter_test: f32 = unity_edges.iter().map(|&(_, value)| value).sum();

    let ref_percentages: Vec<f32> = clam_edges
        .iter()
        .map(|&(_, val)| val / perimeter_ref)
        .collect();

    let test_percentages: Vec<f32> = unity_edges
        .iter()
        .map(|&(_, val)| val / perimeter_test)
        .collect();

    let distortion: f32 = ref_percentages
        .iter()
        .zip(test_percentages.iter())
        .map(|(&x, &y)| (y - x).abs())
        .sum();
    return distortion as f64;
}

pub fn get_unity_triangle<'a>(
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

pub fn write_results(outpath: &PathBuf, results: &Vec<String>) {
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
