use core::{ffi, num};
use std::collections::HashSet;
use std::ffi::{c_char, CStr};

use abd_clam::Cluster;
use distances::Number;

use crate::ffi_impl::cleanup::Cleanup;
use crate::{
    debug,
    utils::{
        error::FFIError,
        helpers,
        types::{InHandlePtr, Vertexf32},
    },
    CBFnNameSetter, CBFnNodeVisitor,
};

use super::cluster_data::ClusterData;

/// Function that calls the `for_each_dft` method on the handle
///
/// # Safety
///
/// This function is unsafe because it dereferences the pointer passed to it
///
/// # Arguments
///
/// * `ptr` - A pointer to the handle
/// * `node_visitor` - A function pointer to the node visitor function
/// * `start_node` - A pointer to the start node
/// * `max_depth` - The maximum depth to traverse
///
/// # Returns
///
/// An `FFIError` enum
pub unsafe fn for_each_dft_impl(
    ptr: InHandlePtr,
    node_visitor: CBFnNodeVisitor,
    offset : usize,
    cardinality : usize,
    max_depth: i32,
) -> FFIError {
    if let Some(handle) = ptr {
        if offset != 0 && cardinality != 0 {
            // let c_str = unsafe { CStr::from_ptr(start_node) };
            // let r_str = c_str.to_str().unwrap();
            return handle.for_each_dft(node_visitor, offset, cardinality, max_depth);
        } else {
            return FFIError::InvalidStringPassed;
        }
    }

    FFIError::NullPointerPassed
}

/// Function that calls the `set_names` method on the handle
///
/// # Safety
///
/// This function is unsafe because it dereferences the pointer passed to it
///
/// # Arguments
///
/// * `ptr` - A pointer to the handle
/// * `node_visitor` - A function pointer to the name setter function
/// * `start_node` - A pointer to the start node
///
/// # Returns
///
/// An `FFIError` enum
pub unsafe fn set_names_impl(
    ptr: InHandlePtr,
    node_visitor: CBFnNameSetter,
    offset : i32,
    cardinality : i32
) -> FFIError {
    if let Some(handle) = ptr {
            return handle.set_names(node_visitor, offset as usize, cardinality as usize);
    }
    FFIError::NullPointerPassed
}

/// Function that frees a resource that was passed to the FFI from the cluster data
///
/// # Arguments
///
/// * `in_cluster_data` - The input cluster data
/// * `out_cluster_data` - The output cluster data
///
/// # Returns
///
/// An `FFIError` enum
pub fn free_resource<T: Clone + Cleanup>(
    in_cluster_data: Option<&T>,
    out_cluster_data: Option<&mut T>,
) -> FFIError {
    if let Some(in_data) = in_cluster_data {
        if let Some(out_data) = out_cluster_data {
            *out_data = in_data.clone();
            out_data.free();
            FFIError::Ok
        } else {
            FFIError::NullPointerPassed
        }
    } else {
        FFIError::NullPointerPassed
    }
}

/// Function that returns the tree height of the handle
///
/// # Safety
///
/// This function is unsafe because it dereferences the pointer passed to it
///
/// # Arguments
///
/// * `ptr` - A pointer to the handle
///
/// # Returns
///
/// The tree height as an `i32`
pub unsafe fn tree_height_impl(ptr: InHandlePtr) -> i32 {
    if let Some(handle) = ptr {
        return handle.tree_height() + 1;
    }
    debug!("handle not created");

    0
}

/// Function that returns the tree cardinality of the handle
///
/// # Safety
///
/// This function is unsafe because it dereferences the pointer passed to it
///
/// # Arguments
///
/// * `ptr` - A pointer to the handle
///
/// # Returns
///
/// The tree cardinality as an `i32`
pub unsafe fn tree_cardinality_impl(ptr: InHandlePtr) -> usize {
    if let Some(handle) = ptr {
        if let Some(tree) = handle.tree() {
            return tree.cardinality();
        }
    }
    debug!("handle not created");
    0
}

/// Function that returns the vertex degree of a cluster in the handle
///
/// # Safety
///
/// This function is unsafe because it dereferences the pointer passed to it
///
/// # Arguments
///
/// * `ptr` - A pointer to the handle
/// * `cluster_id` - A pointer to the cluster id
///
/// # Returns
///
/// The vertex degree as an `i32` or -1 if the handle is not created
pub unsafe fn vertex_degree_impl(ptr: InHandlePtr, offset : i32, cardinality : i32) -> i32 {
    if let Some(handle) = ptr {
        if let Some(clam_graph) = handle.clam_graph() {
            // let cluster_id = helpers::c_char_to_string(cluster_id);
            if let Ok(cluster) = handle.get_cluster(offset as usize, cardinality as usize) {
                if let Ok(degree) = clam_graph.vertex_degree(cluster) {
                    return degree as i32;
                }
            }
        }
    }
    debug!("handle not created");
    -1
}

/// Function that returns the cluster label of a cluster in the handle
///
/// # Safety
///
/// This function is unsafe because it dereferences the pointer passed to it
///
/// # Arguments
///
/// * `ptr` - A pointer to the handle
/// * `cluster_id` - A pointer to the cluster id
///
/// # Returns
///
/// The cluster label as an `i32` or -1 if the handle is not created
pub unsafe fn get_cluster_label_impl(ptr: InHandlePtr,  offset : usize, cardinality: usize) -> i32 {
    // If the handle and label exist
    if let Some(handle) = ptr {
        if let Some(labels) = handle.labels() {
            // Get the cluster id as a string
            // let cluster_id = helpers::c_char_to_string(cluster_id);
            // If the cluster exists in the handle, get the cluster label
            if let Ok(cluster) = handle.get_cluster(offset, cardinality) {
                let num_unique_labels = {
                    let unique_labels: HashSet<_> = labels.iter().cloned().collect();
                    unique_labels.len()
                };
                // Get the label colors
                let colors = helpers::label_colors();
                if num_unique_labels > colors.len() {
                    return -1;
                }
                // Get the dominant label of the cluster and return it or -1 if it doesn't exist
                match calc_cluster_dominant_label(cluster, labels, num_unique_labels, &colors) {
                    Some(label) => {
                        return label as i32;
                    }
                    None => {
                        return -1;
                    }
                }
                // if let Ok(degree) = clam_graph.vertex_degree(cluster) {
                //     return degree as i32;
                // }
            }
        }
    }
    debug!("handle not created");
    -1
}

/// Function that returns the max vertex degree of the handle
///
/// # Safety
///
/// This function is unsafe because it dereferences the pointer passed to it
///
/// # Arguments
///
/// * `ptr` - A pointer to the handle
///
/// # Returns
///
/// The max vertex degree as an `i32` or -1 with a debug message based on where the error occurred
pub unsafe fn max_vertex_degree_impl(ptr: InHandlePtr) -> i32 {
    // If the handle exists
    if let Some(handle) = ptr {
        // If the tree exists
        if handle.tree().is_some() {
            // If the clam graph exists
            if let Some(graph) = handle.clam_graph() {
                // Get the max vertex degree of the graph
                let mut max_degree = -1;
                for c in graph.ordered_clusters() {
                    let vertex_degree = graph.vertex_degree(c).unwrap_or_else(|_| {
                        unreachable!(
                            "We are iterating through clusters in graph so it must be there"
                        )
                    });
                    if vertex_degree as i64 > max_degree {
                        max_degree = vertex_degree as i64;
                    }
                }
                return max_degree as i32;
            }
            debug!("clam graph not built max vertex degree impl");
            return -1;
        }
        debug!("tree not built max vertex degree impl");
    }
    debug!("root not built max vertex degree impl");
    -1
}

/// Function that returns the maximum leaf-to-root distance of each cluster in the handle
///
/// # Safety
///
/// This function is unsafe because it dereferences the pointer passed to it
///
/// # Arguments
///
/// * `ptr` - A pointer to the handle
///
/// # Returns
///
/// The max distance to leaf as an `f32` or -1.0 otherwise.
pub unsafe fn max_lfd_impl(ptr: InHandlePtr) -> f32 {
    // If the handle and tree exist
    if let Some(handle) = ptr {
        if let Some(tree) = handle.tree() {
            // Get the root of the tree
            let clusters = tree.root().subtree();
            // Get the max leaf-to-root distance of each cluster and return the max of all of them
            let max_lfd = clusters
                .iter()
                .map(|c| c.lfd())
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            return max_lfd as f32;
        }
    }
    -1.0
}

/// Function that colors a provided cluster by the entropy of the cluster
///
/// # Arguments
///
/// * `ptr` - A pointer to the handle
/// * `node_visitor` - A function pointer to the node visitor function
///
/// # Returns
///
/// An `FFIError` enum
pub fn color_clusters_by_entropy_impl(ptr: InHandlePtr, node_visitor: CBFnNodeVisitor) -> FFIError {
    // If the handle and root exist
    if let Some(handle) = ptr {
        if let Some(root) = handle.root() {
            // If the labels exist
            if let Some(labels) = handle.labels() {
                // Color the clusters by entropy
                color_helper(Some(root), labels, node_visitor);
                return FFIError::Ok;
            }
        }
    }
    // Return an error if the handle is not created
    FFIError::HandleInitFailed
}

/// Function that calculates the entropy color of a cluster
///
/// # Arguments
///
/// * `cluster` - The cluster to calculate the entropy color of
/// * `labels` - The labels of the cluster
///
/// # Returns
///
/// A `glam::Vec3` representing the entropy color
fn calc_cluster_entropy_color(cluster: &Vertexf32, labels: &[u8]) -> glam::Vec3 {
    // Get the indices of the cluster
    let indices = cluster.indices();
    let mut entropy = [0; 2];
    // Calculate the entropy of the cluster
    indices.for_each(|i| entropy[labels[i] as usize] += 1);

    // Calculate the total entropy
    let total_entropy: u32 = entropy.iter().sum();

    // Calculate the percentage of inliers and outliers
    let perc_inliers = entropy[0] as f32 / total_entropy as f32;
    let perc_outliers = entropy[1] as f32 / total_entropy as f32;

    // Return the entropy color
    glam::Vec3::new(perc_outliers, perc_inliers, 0.)
}

/// Function that calculates the dominant color of a cluster
///
/// # Arguments
///
/// * `cluster` - The cluster to calculate the dominant color of
/// * `labels` - The labels of the cluster
/// * `num_unique_labels` - The number of unique labels
/// * `color_choices` - The color choices
///
/// # Returns
///
/// A `Result` containing the dominant color as a `glam::Vec3` or an error message as a `String`
fn calc_cluster_dominant_color(
    cluster: &Vertexf32,
    labels: &[u8],
    num_unique_labels: usize,
    color_choices: &Vec<glam::Vec3>,
) -> Result<glam::Vec3, String> {
    // Calculate the dominant label of the cluster
    let max_index = calc_cluster_dominant_label(cluster, labels, num_unique_labels, color_choices);

    // Return the dominant color or an error message
    match max_index {
        Some(dom_label) => Ok(color_choices[dom_label as usize]),
        None => Err("invalid labels? i guess".to_string()),
    }

    // glam::Vec3::new(perc_outliers, perc_inliers, 0.)
}

/// Function that calculates the dominant label of a cluster
///
/// # Arguments
///
/// * `cluster` - The cluster to calculate the dominant label of
/// * `labels` - The labels of the cluster
/// * `num_unique_labels` - The number of unique labels
/// * `color_choices` - The color choices
///
/// # Returns
///
/// An `Option` containing the dominant label as a `usize`
fn calc_cluster_dominant_label(
    cluster: &Vertexf32,
    labels: &[u8],
    num_unique_labels: usize,
    color_choices: &Vec<glam::Vec3>,
) -> Option<usize> {
    let indices = cluster.indices();
    // let unique_values: HashSet<_> = labels.iter().cloned().collect();

    // set up a vector to store the entropy
    let mut entropy = vec![0; num_unique_labels];
    indices.for_each(|i| entropy[labels[i] as usize] += 1);
    // iterate through the entropy and get the max index, which is the dominant label
    let max_index = entropy
        .iter()
        .enumerate()
        .max_by_key(|&(_, val)| val)
        .map(|(index, _)| index);

    max_index

    // glam::Vec3::new(perc_outliers, perc_inliers, 0.)
}

/// Function that colors the clusters by the dominant label of the cluster
///
/// # Arguments
///
/// * `ptr` - A pointer to the handle
/// * `node_visitor` - A function pointer to the node visitor function
fn color_helper(root: Option<&Vertexf32>, labels: &[u8], node_visitor: CBFnNodeVisitor) {
    // If the root exists
    if let Some(cluster) = root {
        // Get the cluster data
        let mut cluster_data = ClusterData::from_clam(cluster);
        cluster_data.color = calc_cluster_entropy_color(cluster, labels);

        // Visit the node
        node_visitor(Some(&cluster_data));

        // If the cluster has children, color the children
        if let Some([left, right]) = cluster.children() {
            color_helper(Some(left), labels, node_visitor);
            color_helper(Some(right), labels, node_visitor);
        }
    }
}

/// Function that colors the clusters by the dominant label of the cluster
///
/// # Arguments
///
/// * `ptr` - A pointer to the handle
/// * `node_visitor` - A function pointer to the node visitor function
///
/// # Returns
///
/// An `FFIError` enum
pub fn color_clusters_by_dominant_label_impl(
    ptr: InHandlePtr,
    node_visitor: CBFnNodeVisitor,
) -> FFIError {
    // If the handle and root exist
    if let Some(handle) = ptr {
        if let Some(root) = handle.root() {
            // If the labels exist
            if let Some(labels) = handle.labels() {
                // Color the clusters by the dominant label
                let num_unique_labels = {
                    let unique_labels: HashSet<_> = labels.iter().cloned().collect();
                    unique_labels.len()
                };
                // Get the label colors
                let colors = helpers::label_colors();
                if num_unique_labels > colors.len() {
                    return FFIError::TooManyLabels;
                }
                // Color the clusters by the dominant label
                for c in root.subtree() {
                    let mut cluster_data = ClusterData::from_clam(c);
                    if let Ok(color) =
                        calc_cluster_dominant_color(c, labels, num_unique_labels, &colors)
                    {
                        cluster_data.color = color;
                        node_visitor(Some(&cluster_data));
                    } else {
                        return FFIError::ColoringFailed;
                    }
                }
                return FFIError::Ok;
            }
        }
    }

    FFIError::HandleInitFailed
}

/// Function that colors the clusters by the distance to the query
///
/// # Safety
///
/// This function is unsafe because it dereferences the pointer passed to it
///
/// # Arguments
///
/// * `context` - A pointer to the handle
/// * `arr_ptr` - A pointer to the cluster data
/// * `len` - The length of the cluster data
/// * `node_visitor` - A function pointer to the node visitor function
///
/// # Returns
///
/// An `FFIError` enum
pub unsafe fn color_by_dist_to_query_impl(
    context: InHandlePtr,
    arr_ptr: *mut ClusterData,
    len: i32,
    node_visitor: CBFnNodeVisitor,
) -> FFIError {
    // If the handle and cluster data exist
    // if let Some(handle) = context {
    //     if arr_ptr.is_null() {
    //         return FFIError::NullPointerPassed;
    //     }
    //     // Get the cluster data from the pointer
    //     let arr = std::slice::from_raw_parts(arr_ptr, len as usize);

    //     let mut ids = Vec::new();

    //     // Get the ids of the clusters
    //     for node in arr {
    //         ids.push(node.id.as_string().unwrap());
    //     }

    //     // Color the clusters by the distance to the query
    //     handle.color_by_dist_to_query(ids.as_slice(), node_visitor)
    // } else {
    //     FFIError::NullPointerPassed
    // }
    FFIError::NotImplemented
}

/// Function that returns the distance to the other cluster
///
/// # Safety
///
/// This function is unsafe because it dereferences the pointer passed to it
///
/// # Arguments
///
/// * `ptr` - A pointer to the handle
/// * `node_name1` - A pointer to the first cluster name
/// * `node_name2` - A pointer to the second cluster name
///
/// # Returns
///
/// The distance to the other cluster as an `f32` or -1.0 otherwise
pub unsafe fn distance_to_other_impl(
    ptr: InHandlePtr,
    node_name1: *const c_char,
    node_name2: *const c_char,
) -> f32 {
    // If the handle exists
    // if let Some(handle) = ptr {
    //     let node1 = handle.get_cluster_from_string(helpers::c_char_to_string(node_name1));
    //     let node2 = handle.get_cluster_from_string(helpers::c_char_to_string(node_name2));

    //     // Return the distance to the other cluster or -1.0 if it doesn't exist
    //     return if let Ok(node1) = node1 {
    //         if let Ok(node2) = node2 {
    //             node1.distance_to_other(handle.data().unwrap(), node2)
    //         } else {
    //             -1f32
    //         }
    //     } else {
    //         -1f32
    //     };
    // }

    -1f32
    // FFIError::NotImplemented
}

// pub unsafe fn test_cakes_rnn_query_impl(
//     ptr: InHandlePtr,
//     search_radius: f32,
//     node_visitor: CBFnNodeVisitor,
// ) -> FFIError {
//     if let Some(handle) = ptr {
//         let num_queries = 1;

//         for j in 0..1000 {
//             let queries = abd_clam::utils::helpers::gen_data_f32(num_queries, 10, 0., 1., j);
//             let queries = queries.iter().collect::<Vec<_>>();
//             for i in 0..num_queries {
//                 let (query, radius, _) = (&queries[i], search_radius, 10);
//                 handle.set_current_query(query);
//                 let rnn_results = handle.rnn_search(query, radius);
//                 match rnn_results {
//                     Ok((confirmed, straddlers)) => {
//                         if straddlers.len() < 5 || confirmed.len() < 5 {
//                             continue;
//                         }

//                         for (cluster, dist) in &confirmed {
//                             let mut baton = ClusterDataWrapper::from_cluster(cluster);
//                             baton.data_mut().dist_to_query = *dist;
//                             baton.data_mut().set_color(glam::Vec3 {
//                                 x: 0f32,
//                                 y: 1f32,
//                                 z: 0f32,
//                             });
//                             node_visitor(Some(baton.data()));
//                         }

//                         for (cluster, dist) in &straddlers {
//                             let mut baton = ClusterDataWrapper::from_cluster(cluster);
//                             baton.data_mut().dist_to_query = *dist;

//                             baton.data_mut().set_color(glam::Vec3 {
//                                 x: 0f32,
//                                 y: 1f32,
//                                 z: 1f32,
//                             });
//                             node_visitor(Some(baton.data()));
//                         }

//                         return FFIError::Ok;
//                     }
//                     Err(_) => {
//                         debug!("rnn failes");
//                     }
//                 }
//             }
//         }
//     }

//     return FFIError::Ok;
// }
