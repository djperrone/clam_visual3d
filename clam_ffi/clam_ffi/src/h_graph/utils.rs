use std::collections::HashSet;

use abd_clam::{graph::Edge, Cluster, Dataset, Instance, MinCardinality, Tree, VecDataset};
use distances::{number::Float, Number};
use glam::{vec3, Vec3};

use crate::utils::types::{DataSetf32, Graphf32, Treef32, Vertexf32};

pub fn vec3_distance_normalized_scaled(p1: &Vec3, p2: &Vec3, scalar: f32, max_len: f32) -> f32 {
    return p1.distance(*p2) / max_len.max(f32::MIN) * scalar;
}

pub fn cluster_distance_normalized_scaled(
    p1: &Vertexf32,
    p2: &Vertexf32,
    data: &DataSetf32,
    scalar: f32,
    max_len: f32,
) -> f32 {
    return p1.distance_to_other(data, p2) / max_len.max(f32::MIN) * scalar;
}

pub fn cluster_center_distance_normalized_scaled(
    p1: &Vertexf32,
    p2: &Vertexf32,
    tree: &Treef32,
    scalar: f32,
    max_len: f32,
) -> Option<f32> {
    let center_data = tree.data().data().get(p1.arg_center())?;
    let left_data = tree.data().data().get(p2.arg_center())?;

    let distance = tree.data().metric()(center_data, left_data);

    return Some(distance / max_len.max(f32::MIN) * scalar);
}

pub fn max_edge_len<'a>(edges: &HashSet<Edge<'a, f32>>) -> f32 {
    match edges
        .iter()
        .reduce(|cur_max: &Edge<'a, f32>, val: &Edge<'a, f32>| {
            if cur_max.distance() > val.distance() {
                cur_max
            } else {
                val
            }
        }) {
        Some(spring) => spring.distance(),
        None => 1.0,
    }

    // max_edge_len
}

/// Generate a dataset from the given data.
pub fn gen_dataset_from<T: Number, U: Number, M: Instance>(
    data: Vec<Vec<T>>,
    metric: fn(&Vec<T>, &Vec<T>) -> U,
    metadata: Vec<M>,
) -> VecDataset<Vec<T>, U, M> {
    let name = "test".to_string();
    VecDataset::new(name, data, metric, false)
        .assign_metadata(metadata)
        .unwrap_or_else(|_| unreachable!())
}

/// Euclidean distance between two vectors.
pub fn euclidean<T: Number, F: Float>(x: &Vec<T>, y: &Vec<T>) -> F {
    distances::vectors::euclidean(x, y)
}

pub fn calc_triangle_distances(
    cluster: &Vertexf32,
    left: &Vertexf32,
    right: &Vertexf32,
    tree: &Treef32,
    scalar: f32,
    max_len: f32,
) -> Option<(f32, f32, f32)> {
    // let dist_between_children = left.distance_to_other(tree.data(), right);
    let dist_between_children =
        cluster_distance_normalized_scaled(left, right, tree.data(), scalar, max_len);
    // let center_data = tree.data().data().get(cluster.arg_center())?;
    // let left_data = tree.data().data().get(left.arg_center())?;
    // let right_data = tree.data().data().get(right.arg_center())?;

    // let dist_to_left_child = tree.data().metric()(center_data, left_data);
    // let dist_to_right_child = tree.data().metric()(center_data, right_data);

    let dist_to_left_child =
        cluster_center_distance_normalized_scaled(cluster, left, tree, scalar, max_len)?;
    let dist_to_right_child =
        cluster_center_distance_normalized_scaled(cluster, right, tree, scalar, max_len)?;

    Some((
        dist_between_children,
        dist_to_left_child,
        dist_to_right_child,
    ))
}

pub fn calc_circle_intersection(
    p1: &Vec3,
    r1: f32,
    p2: &Vec3,
    r2: f32,
    d: f32,
) -> Result<Vec3, String> {
    if d > r1 + r2 || d < (r1 - r2).abs() {
        return Err("Not a valid circle".to_string());
    }

    let a = (r1 * r1 - r2 * r2 + d * d) / (2. * d);
    let h = (r1 * r1 - a * a).sqrt();
    let x0 = p1.x + a * (p2.x - p1.x) / d;
    let y0 = p1.y + a * (p2.y - p1.y) / d;

    let x3 = x0 + h * (p2.y - p1.y) / d;
    let y3 = y0 - h * (p2.x - p1.x) / d;
    let x4 = x0 - h * (p2.y - p1.y) / d;
    let y4 = y0 + h * (p2.x - p1.x) / d;

    if y3 < y4 {
        return Ok(vec3(x3, y3, p1.z));
    } else {
        return Ok(vec3(x4, y4, p1.z));
    }
}

pub fn get_cluster_from_graph<'a>(graph: &'a Graphf32, name: &String) -> Option<&'a Vertexf32> {
    graph
        .ordered_clusters()
        .iter()
        .map(|c| *c)
        .find(|c| c.name() == *name)
}

pub fn parse_cluster_name(name: &str) -> Result<(usize, usize), String> {
    if let Ok((offset, cardinality)) = crate::utils::helpers::parse_cluster_name(name) {
        return Ok((offset, cardinality));
    } else {
        return Err("Could not parse cluster name".to_string());
    }
}

pub fn get_cluster_from_tree<'a>(
    tree: &'a Treef32,
    offset: usize,
    cardinality: usize,
) -> Result<&'a Vertexf32, String> {
    if let Some(cluster) = tree.get_cluster(offset, cardinality) {
        return Ok(cluster);
    } else {
        return Err("Tree does not contain cluster".to_string());
    }
}

pub fn get_cluster<'a>(tree: &'a Treef32, name: &str) -> Result<&'a Vertexf32, String> {
    let (offset, cardinality) = parse_cluster_name(name)?;
    get_cluster_from_tree(tree, offset, cardinality)
}

pub fn get_children<'a>(cluster: &'a Vertexf32) -> Result<[&'a Vertexf32; 2], String> {
    if let Some(children) = cluster.children() {
        return Ok(children);
    } else {
        return Err("Cluster has no children".to_string());
    }
}
