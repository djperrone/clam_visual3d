use core::num;
use std::collections::HashSet;
use std::ffi::{c_char, CStr};

use distances::Number;

use crate::ffi_impl::cleanup::Cleanup;
use crate::{
    debug,
    utils::{
        error::FFIError,
        helpers,
        types::{Clusterf32, InHandlePtr},
    },
    CBFnNameSetter, CBFnNodeVisitor,
};

use super::{cluster_data::ClusterData, cluster_data_wrapper::ClusterDataWrapper};

pub unsafe fn for_each_dft_impl(
    ptr: InHandlePtr,
    node_visitor: CBFnNodeVisitor,
    start_node: *const c_char,
    max_depth: i32,
) -> FFIError {
    if let Some(handle) = ptr {
        if !start_node.is_null() {
            let c_str = unsafe { CStr::from_ptr(start_node) };
            let r_str = c_str.to_str().unwrap();
            return handle.for_each_dft(node_visitor, r_str.to_string(), max_depth);
        } else {
            return FFIError::InvalidStringPassed;
        }
    }

    FFIError::NullPointerPassed
}

pub unsafe fn set_names_impl(
    ptr: InHandlePtr,
    node_visitor: CBFnNameSetter,
    start_node: *const c_char,
) -> FFIError {
    if let Some(handle) = ptr {
        return if !start_node.is_null() {
            let c_str = unsafe { CStr::from_ptr(start_node) };
            let r_str = c_str.to_str().unwrap();
            handle.set_names(node_visitor, r_str.to_string())
        } else {
            FFIError::InvalidStringPassed
        };
    }
    FFIError::NullPointerPassed
}

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

pub unsafe fn tree_height_impl(ptr: InHandlePtr) -> i32 {
    if let Some(handle) = ptr {
        return handle.tree_height() + 1;
    }
    debug!("handle not created");

    0
}

pub unsafe fn tree_cardinality_impl(ptr: InHandlePtr) -> i32 {
    if let Some(handle) = ptr {
        if let Some(tree) = handle.get_tree() {
            return tree.cardinality() as i32;
        }
    }
    debug!("handle not created");
    -1
}

pub unsafe fn vertex_degree_impl(ptr: InHandlePtr, cluster_id: *const c_char) -> i32 {
    if let Some(handle) = ptr {
        if let Some(clam_graph) = handle.clam_graph() {
            let cluster_id = helpers::c_char_to_string(cluster_id);
            if let Ok(cluster) = handle.get_cluster_from_string(cluster_id) {
                if let Ok(degree) = clam_graph.vertex_degree(cluster) {
                    return degree as i32;
                }
            }
        }
    }
    debug!("handle not created");
    -1
}

pub unsafe fn get_cluster_label_impl(ptr: InHandlePtr, cluster_id: *const c_char) -> i32 {
    if let Some(handle) = ptr {
        if let Some(labels) = handle.labels() {
            let cluster_id = helpers::c_char_to_string(cluster_id);
            if let Ok(cluster) = handle.get_cluster_from_string(cluster_id) {
                let num_unique_labels = {
                    let unique_labels: HashSet<_> = labels.iter().cloned().collect();
                    unique_labels.len()
                };

                let colors = helpers::label_colors();
                if num_unique_labels > colors.len() {
                    return -1;
                }
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

pub unsafe fn max_vertex_degree_impl(ptr: InHandlePtr) -> i32 {
    if let Some(handle) = ptr {
        if handle.get_tree().is_some() {
            if let Some(graph) = handle.clam_graph() {
                let mut max_degree = -1;
                for c in graph.clusters() {
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

pub unsafe fn max_lfd_impl(ptr: InHandlePtr) -> f32 {
    if let Some(handle) = ptr {
        if let Some(tree) = handle.tree() {
            let clusters = tree.root().subtree();
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

pub fn color_clusters_by_entropy_impl(ptr: InHandlePtr, node_visitor: CBFnNodeVisitor) -> FFIError {
    if let Some(handle) = ptr {
        if let Some(root) = handle.root() {
            if let Some(labels) = handle.labels() {
                color_helper(Some(root), labels, node_visitor);
                return FFIError::Ok;
            }
        }
    }
    FFIError::HandleInitFailed
}

fn calc_cluster_entropy_color(cluster: &Clusterf32, labels: &[u8]) -> glam::Vec3 {
    let indices = cluster.indices();
    let mut entropy = [0; 2];
    indices.for_each(|i| entropy[labels[i] as usize] += 1);

    let total_entropy: u32 = entropy.iter().sum();

    let perc_inliers = entropy[0] as f32 / total_entropy as f32;
    let perc_outliers = entropy[1] as f32 / total_entropy as f32;

    glam::Vec3::new(perc_outliers, perc_inliers, 0.)
}

fn calc_cluster_dominant_color(
    cluster: &Clusterf32,
    labels: &[u8],
    num_unique_labels: usize,
    color_choices: &Vec<glam::Vec3>,
) -> Result<glam::Vec3, String> {
    let max_index = calc_cluster_dominant_label(cluster, labels, num_unique_labels, color_choices);

    match max_index {
        Some(dom_label) => Ok(color_choices[dom_label as usize]),
        None => Err("invalid labels? i guess".to_string()),
    }

    // glam::Vec3::new(perc_outliers, perc_inliers, 0.)
}

fn calc_cluster_dominant_label(
    cluster: &Clusterf32,
    labels: &[u8],
    num_unique_labels: usize,
    color_choices: &Vec<glam::Vec3>,
) -> Option<usize> {
    let indices = cluster.indices();
    // let unique_values: HashSet<_> = labels.iter().cloned().collect();

    let mut entropy = vec![0; num_unique_labels];
    indices.for_each(|i| entropy[labels[i] as usize] += 1);
    let max_index = entropy
        .iter()
        .enumerate()
        .max_by_key(|&(_, val)| val)
        .map(|(index, _)| index);

    max_index

    // glam::Vec3::new(perc_outliers, perc_inliers, 0.)
}
fn color_helper(root: Option<&Clusterf32>, labels: &[u8], node_visitor: CBFnNodeVisitor) {
    if let Some(cluster) = root {
        let mut cluster_data = ClusterDataWrapper::from_cluster(cluster);
        cluster_data.data_mut().color = calc_cluster_entropy_color(cluster, labels);

        node_visitor(Some(cluster_data.data()));

        if let Some([left, right]) = cluster.children() {
            color_helper(Some(left), labels, node_visitor);
            color_helper(Some(right), labels, node_visitor);
        }
    }
}

pub fn color_clusters_by_dominant_label_impl(
    ptr: InHandlePtr,
    node_visitor: CBFnNodeVisitor,
) -> FFIError {
    if let Some(handle) = ptr {
        if let Some(root) = handle.root() {
            if let Some(labels) = handle.labels() {
                let num_unique_labels = {
                    let unique_labels: HashSet<_> = labels.iter().cloned().collect();
                    unique_labels.len()
                };

                let colors = helpers::label_colors();
                if num_unique_labels > colors.len() {
                    return FFIError::TooManyLabels;
                }

                for c in root.subtree() {
                    let mut cluster_data = ClusterDataWrapper::from_cluster(c);
                    if let Ok(color) =
                        calc_cluster_dominant_color(c, labels, num_unique_labels, &colors)
                    {
                        cluster_data.data_mut().color = color;
                        node_visitor(Some(cluster_data.data()));
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

pub unsafe fn color_by_dist_to_query_impl(
    context: InHandlePtr,
    arr_ptr: *mut ClusterData,
    len: i32,
    node_visitor: CBFnNodeVisitor,
) -> FFIError {
    if let Some(handle) = context {
        if arr_ptr.is_null() {
            return FFIError::NullPointerPassed;
        }
        let arr = std::slice::from_raw_parts(arr_ptr, len as usize);

        let mut ids = Vec::new();
        for node in arr {
            ids.push(node.id.as_string().unwrap());
        }

        handle.color_by_dist_to_query(ids.as_slice(), node_visitor)
    } else {
        FFIError::NullPointerPassed
    }
}

pub unsafe fn distance_to_other_impl(
    ptr: InHandlePtr,
    node_name1: *const c_char,
    node_name2: *const c_char,
) -> f32 {
    if let Some(handle) = ptr {
        let node1 = handle.get_cluster_from_string(helpers::c_char_to_string(node_name1));
        let node2 = handle.get_cluster_from_string(helpers::c_char_to_string(node_name2));

        return if let Ok(node1) = node1 {
            if let Ok(node2) = node2 {
                node1.distance_to_other(handle.data().unwrap(), node2)
            } else {
                -1f32
            }
        } else {
            -1f32
        };
    }

    -1f32
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
