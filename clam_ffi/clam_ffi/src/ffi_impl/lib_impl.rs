use std::ffi::{c_char, CStr};

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
            let c_str = unsafe {
                // assert!(!start_node.is_null());

                CStr::from_ptr(start_node)
            };
            let r_str = c_str.to_str().unwrap();
            // debug!("start node name {}", r_str);

            // return Handle::from_ptr(ptr).for_each_dft(node_visitor, r_str.to_string());
            return handle.for_each_dft(node_visitor, r_str.to_string(), max_depth);
        } else {
            return FFIError::InvalidStringPassed;
        }
    }

    return FFIError::NullPointerPassed;
}

pub unsafe fn set_names_impl(
    ptr: InHandlePtr,
    node_visitor: CBFnNameSetter,
    start_node: *const c_char,
) -> FFIError {
    if let Some(handle) = ptr {
        if !start_node.is_null() {
            let c_str = unsafe {
                // assert!(!start_node.is_null());

                CStr::from_ptr(start_node)
            };
            let r_str = c_str.to_str().unwrap();
            // debug!("start node name {}", r_str);

            // return Handle::from_ptr(ptr).for_each_dft(node_visitor, r_str.to_string());
            return handle.set_names(node_visitor, r_str.to_string());
        } else {
            return FFIError::InvalidStringPassed;
        }
    }

    return FFIError::NullPointerPassed;
}

pub fn free_resource<T: Clone + Cleanup>(
    in_cluster_data: Option<&T>,
    out_cluster_data: Option<&mut T>,
) -> FFIError {
    // if data.is_none() {
    // }

    return if let Some(in_data) = in_cluster_data {
        if let Some(out_data) = out_cluster_data {
            *out_data = in_data.clone();
            out_data.free();
            FFIError::Ok
        } else {
            FFIError::NullPointerPassed
        }
    } else {
        FFIError::NullPointerPassed
    };
}

pub unsafe fn tree_height_impl(ptr: InHandlePtr) -> i32 {
    // Handle::from_ptr(ptr).get_num_nodes() + 1

    if let Some(handle) = ptr {
        // debug!("cardinality: {}", handle.tree_height() + 1);

        return handle.tree_height() + 1;
    }
    debug!("handle not created");

    return 0;
}

pub unsafe fn tree_cardinality_impl(ptr: InHandlePtr) -> i32 {
    // Handle::from_ptr(ptr).get_num_nodes() + 1

    if let Some(handle) = ptr {
        // debug!("cardinality: {}", handle.tree_height() + 1);

        if let Some(tree) = handle.get_tree() {
            return tree.cardinality() as i32;
        }
    }
    debug!("handle not created");

    return -1;
}

pub unsafe fn max_lfd_impl(ptr: InHandlePtr) -> f32 {
    // Handle::from_ptr(ptr).get_num_nodes() + 1

    if let Some(handle) = ptr {
        if let Some(cakes) = handle.cakes() {
            let clusters = cakes.trees().first().unwrap().root().subtree();
            let lfd = clusters.first().unwrap().lfd();
            let max_lfd = clusters
                .iter()
                .map(|c| c.lfd())
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            return max_lfd as f32;
        }
    }
        return -1.0;
}

pub fn color_clusters_by_label_impl(ptr: InHandlePtr, node_visitor: CBFnNodeVisitor) -> FFIError {
    if let Some(handle) = ptr {
        if let Some(root) = handle.root() {
            if let Some(labels) = handle.labels() {
                // if *(labels.iter().max().unwrap()) > (1) {
                //     // need error message for labels not matching
                //     return FFIError::HandleInitFailed;
                // }
                color_helper(Some(root), &labels, node_visitor);
                return FFIError::Ok;
            }
        }
    }
    return FFIError::HandleInitFailed;
}

fn color_helper(root: Option<&Clusterf32>, labels: &Vec<bool>, node_visitor: CBFnNodeVisitor) {
    if let Some(cluster) = root {
        let indices = cluster.indices();
        let mut entropy = vec![0; 2];

        indices.for_each(|i| entropy[labels[i] as usize] += 1);

        let total_entropy: u32 = entropy.iter().sum();

        let perc_inliers = entropy[0] as f32 / total_entropy as f32;
        let perc_outliers = entropy[1] as f32 / total_entropy as f32;
        debug!("inliers: {}, outliers: {}", perc_inliers, perc_outliers);
        let mut cluster_data = ClusterDataWrapper::from_cluster(cluster);
        cluster_data.data_mut().color = glam::Vec3::new(perc_outliers, perc_inliers, 0.);
        node_visitor(Some(cluster_data.data()));

        if let Some([left, right]) = cluster.children() {
            color_helper(Some(left), labels, node_visitor);
            color_helper(Some(right), labels, node_visitor);
        }
        // debug!("here4");
    }
}
// pub unsafe fn test_mod_struct(context: InHandlePtr, ptr: *mut ClusterData) {
//     if let Some(handle) = context {
//         let mut out_node = unsafe { &mut *ptr };
//         out_node.arg_center = 5;
//         match out_node.id.as_string() {
//             Ok(path) => match handle.find_node(path) {
//                 Ok(cluster_data) => {
//                     out_node.set_from_clam(&cluster_data);
//                     if let Some(query) = handle.get_current_query() {
//                         out_node.dist_to_query =
//                             cluster_data.distance_to_instance(handle.data().unwrap(), query);
//                     }
//                     // return FFIError::Ok;
//                 }
//                 Err(e) => {
//                     debug!("error {:?}", e);
//                     // return e;
//                 }
//             },
//             Err(e) => {
//                 debug!("error {:?}", e);
//                 // return e;
//             }
//         }
//     }
// }

// pub unsafe fn get_cluster_data_impl(
//     context: InHandlePtr,
//     incoming: Option<&ClusterData>,
//     outgoing: Option<&mut ClusterData>,
// ) -> FFIError {
//     if let Some(handle) = context {
//         if let Some(in_node) = incoming {
//             if let Some(out_node) = outgoing {
//                 *out_node = *in_node;

//                 match out_node.id.as_string() {
//                     Ok(path) => match handle.find_node(path) {
//                         Ok(cluster_data) => {
//                             out_node.set_from_clam(&cluster_data);
//                             if let Some(query) = handle.get_current_query() {
//                                 out_node.dist_to_query = cluster_data
//                                     .distance_to_instance(handle.data().unwrap(), query);
//                             }
//                             return FFIError::Ok;
//                         }
//                         Err(e) => {
//                             debug!("error {:?}", e);
//                             return e;
//                         }
//                     },
//                     Err(e) => {
//                         debug!("error {:?}", e);
//                         return e;
//                     }
//                 }
//             }
//         }
//     }
//     return FFIError::NullPointerPassed;
// }

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
        // debug!("creating string arr");
        let arr = std::slice::from_raw_parts(arr_ptr, len as usize);

        let mut ids = Vec::new();
        for node in arr {
            ids.push(node.id.as_string().unwrap());
        }

        let err = handle.color_by_dist_to_query(ids.as_slice(), node_visitor);
        // debug!("color result {:?}", err);
        return err;
    } else {
        return FFIError::NullPointerPassed;
    }
    // return FFIError::Ok;
}

pub unsafe fn distance_to_other_impl(
    ptr: InHandlePtr,
    node_name1: *const c_char,
    node_name2: *const c_char,
) -> f32 {
    if let Some(handle) = ptr {
        let node1 = handle.get_cluster(helpers::c_char_to_string(node_name1));
        let node2 = handle.get_cluster(helpers::c_char_to_string(node_name2));

        if let Ok(node1) = node1 {
            if let Ok(node2) = node2 {
                let distance = node1.distance_to_other(handle.data().unwrap(), node2);
                return distance;
            } else {
                return -1f32;
            }
        } else {
            return -1f32;
        }
    }

    return -1f32;
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
