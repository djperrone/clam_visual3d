//need to use partial struct to pass by reference and access sub structs
// pass by pointer with classes but cant seem to access sub structs

use std::ffi::c_char;
mod ffi_impl;
mod file_io;
mod graph;
mod handle;
mod tests;
mod tree_layout;
mod utils;

use crate::ffi_impl::lib_impl::{free_resource, max_lfd_impl, max_vertex_degree_impl, vertex_degree_impl};
use crate::ffi_impl::tree_startup_data_ffi::TreeStartupDataFFI;
use crate::file_io::load_save::save_cakes_single_impl;
use ffi_impl::{
    cluster_data::ClusterData,
    cluster_ids::ClusterIDs,
    lib_impl::{
        color_by_dist_to_query_impl, distance_to_other_impl, for_each_dft_impl, set_names_impl,
        tree_cardinality_impl, tree_height_impl,
    },
    string_ffi::StringFFI,
};
use graph::entry::{
    get_num_edges_in_graph_impl, init_force_directed_graph_impl, init_graph_vertices_impl,
    physics_update_async_impl, shutdown_physics_impl,
};
use tree_layout::entry_point::{draw_hierarchy_impl, draw_hierarchy_offset_from_impl};
use utils::{
    debug,
    distances::DistanceMetric,
    error::FFIError,
    // helpers,
    types::{InHandlePtr, OutHandlePtr},
};

use crate::handle::entry_point::{
    init_clam_impl, init_clam_struct_impl, load_cakes_struct_impl, shutdown_clam_impl,
};
use crate::utils::scoring_functions::ScoringFunction;

type CBFnNodeVisitor = extern "C" fn(Option<&ClusterData>) -> ();
type CBFnNameSetter = extern "C" fn(Option<&ClusterIDs>) -> ();
type CBFnNodeVisitorMut = extern "C" fn(Option<&mut ClusterData>) -> ();

#[no_mangle]
pub unsafe extern "C" fn create_cluster_data(
    ptr: InHandlePtr,
    id: *const c_char,
    outgoing: Option<&mut ClusterData>,
) -> FFIError {
    if let Some(handle) = ptr {
        let outgoing = outgoing.unwrap();
        let id = utils::helpers::c_char_to_string(id);
        // let data = Box::new(ClusterData::default());

        // match out_node.id.as_string() {
        return match handle.get_cluster_from_string(id) {
            Ok(cluster) => {
                let cluster_data = ClusterData::from_clam(cluster);

                *outgoing = cluster_data;
                FFIError::Ok
            }
            Err(_) => FFIError::InvalidStringPassed,
        };
    }
    return FFIError::NullPointerPassed;
}

#[no_mangle]
pub unsafe extern "C" fn alloc_string(
    value: *const c_char,
    outgoing: Option<&mut StringFFI>,
) -> FFIError {
    let outgoing = outgoing.unwrap();
    let value = utils::helpers::c_char_to_string(value);
    let data = StringFFI::new(value);

    *outgoing = data;
    return FFIError::Ok;
}

#[no_mangle]
pub extern "C" fn delete_cluster_data(
    in_cluster_data: Option<&ClusterData>,
    out_cluster_data: Option<&mut ClusterData>,
) -> FFIError {
    free_resource(in_cluster_data, out_cluster_data)
    // if data.is_none() {
    // }
}

#[no_mangle]
pub unsafe extern "C" fn free_string(
    in_data: Option<&StringFFI>,
    out_data: Option<&mut StringFFI>,
) -> FFIError {
    return free_resource(in_data, out_data);
}

#[no_mangle]
pub unsafe extern "C" fn create_cluster_ids(
    ptr: InHandlePtr,
    id: *const c_char,
    outgoing: Option<&mut ClusterIDs>,
) -> FFIError {
    if let Some(handle) = ptr {
        let outgoing = outgoing.unwrap();
        let id = utils::helpers::c_char_to_string(id);
        // let data = Box::new(ClusterData::default());

        // match out_node.id.as_string() {
        return match handle.get_cluster_from_string(id) {
            Ok(cluster) => {
                let cluster_data = ClusterIDs::from_clam(cluster);

                *outgoing = cluster_data;
                FFIError::Ok
            }
            Err(_) => FFIError::InvalidStringPassed,
        };
    }
    return FFIError::NullPointerPassed;
}

//noinspection ALL
#[no_mangle]
pub extern "C" fn delete_cluster_ids(
    in_cluster_data: Option<&ClusterIDs>,
    out_cluster_data: Option<&mut ClusterIDs>,
) -> FFIError {
    free_resource(in_cluster_data, out_cluster_data)
    // if data.is_none() {
    // }

    // return if let Some(in_data) = in_cluster_data {
    //     if let Some(out_data) = out_cluster_data {
    //         *out_data = *in_data;
    //         out_data.free_ids();
    //         FFIError::Ok
    //     } else {
    //         FFIError::NullPointerPassed
    //     }
    // } else {
    //     FFIError::NullPointerPassed
    // };
}

#[no_mangle]
pub unsafe extern "C" fn set_message(
    msg: *const c_char,
    // in_cluster_data: Option<&ClusterData>,
    out_cluster_data: Option<&mut ClusterData>,
) -> FFIError {
    // if data.is_none() {
    // }

    // if let Some(in_data) = in_cluster_data {
    return if let Some(out_data) = out_cluster_data {
        // *out_data = *in_data;
        // out_data.free_ids();
        let msg_str = StringFFI::c_char_to_string(msg);

        out_data.set_message(msg_str);
        FFIError::Ok
    } else {
        FFIError::NullPointerPassed
    };
    // } else {
    //     return FFIError::NullPointerPassed;
    // }

    // let ctx = data.unwrap();

    // {
    //     unsafe { drop(Box::from_raw(*ctx)) };
    // }

    // *ctx = null_mut();
}

#[repr(C)]
pub struct Context {
    pub foo: bool,
    pub bar: i32,
    pub baz: u64,
}

// ------------------------------------- Startup/Shutdown -------------------------------------

#[no_mangle]
pub unsafe extern "C" fn init_clam(
    ptr: OutHandlePtr,
    data_name: *const u8,
    name_len: i32,
    cardinality: u32,
    distance_metric: DistanceMetric,
) -> FFIError {
    return init_clam_impl(ptr, data_name, name_len, cardinality, distance_metric);
}

#[no_mangle]
pub unsafe extern "C" fn init_clam_struct(
    ptr: OutHandlePtr,
    data: Option<&TreeStartupDataFFI>,
) -> FFIError {
    return init_clam_struct_impl(ptr, data);
}

// #[no_mangle]
// pub unsafe extern "C" fn load_cakes(
//     ptr: OutHandlePtr,
//     data_name: *const u8,
//     name_len: i32,
// ) -> FFIError {
//     return load_cakes_impl(ptr, data_name, name_len);
// }

#[no_mangle]
pub unsafe extern "C" fn load_cakes_struct(
    ptr: OutHandlePtr,
    data: Option<&TreeStartupDataFFI>,
) -> FFIError {
    return load_cakes_struct_impl(ptr, data);
}

#[no_mangle]
pub unsafe extern "C" fn save_cakes(
    ptr: InHandlePtr,
    file_name: *const u8,
    name_len: i32,
) -> FFIError {
    return save_cakes_single_impl(ptr, file_name, name_len);
}
#[no_mangle]
pub unsafe extern "C" fn shutdown_clam(context_ptr: OutHandlePtr) -> FFIError {
    return shutdown_clam_impl(context_ptr);
}

// ------------------------------------- Graph Clam Init -------------------------------------
#[no_mangle]
pub unsafe extern "C" fn init_clam_graph(
    context: InHandlePtr,
    scoring_function: ScoringFunction,
    cluster_selector: CBFnNodeVisitor,
) -> FFIError {
    if let Some(handle) = context {
        handle.init_clam_graph(scoring_function, cluster_selector);
        return FFIError::Ok;
    }
    return FFIError::HandleInitFailed;
}

// -------------------------------------  Tree helpers -------------------------------------

#[no_mangle]
pub unsafe extern "C" fn for_each_dft(
    ptr: InHandlePtr,
    node_visitor: CBFnNodeVisitor,
    start_node: *const c_char,
    max_depth: i32,
) -> FFIError {
    return for_each_dft_impl(ptr, node_visitor, start_node, max_depth);
}

#[no_mangle]
pub unsafe extern "C" fn set_names(
    ptr: InHandlePtr,
    node_visitor: CBFnNameSetter,
    start_node: *const c_char,
) -> FFIError {
    return set_names_impl(ptr, node_visitor, start_node);
}

#[no_mangle]
pub unsafe extern "C" fn tree_height(ptr: InHandlePtr) -> i32 {
    return tree_height_impl(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn tree_cardinality(ptr: InHandlePtr) -> i32 {
    return tree_cardinality_impl(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn vertex_degree(ptr: InHandlePtr, cluster_id: *const c_char) -> i32 {
    return vertex_degree_impl(ptr, cluster_id);
}

#[no_mangle]
pub unsafe extern "C" fn max_vertex_degree(ptr: InHandlePtr) -> i32 {
    return max_vertex_degree_impl(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn max_lfd(ptr: InHandlePtr) -> f32 {
    return max_lfd_impl(ptr);
}

#[no_mangle]
// add recursive bool option and node name
pub unsafe extern "C" fn color_clusters_by_label(
    ptr: InHandlePtr,
    node_visitor: CBFnNodeVisitor,
) -> FFIError {
    return ffi_impl::lib_impl::color_clusters_by_label_impl(ptr, node_visitor);
}
// ------------------------------------- Cluster Helpers -------------------------------------

// #[no_mangle]
// pub unsafe extern "C" fn get_cluster_data(
//     context: InHandlePtr,
//     incoming: Option<&ClusterData>,
//     outgoing: Option<&mut ClusterData>,
// ) -> FFIError {
//     return get_cluster_data_impl(context, incoming, outgoing);
// }

#[no_mangle]
pub unsafe extern "C" fn distance_to_other(
    ptr: InHandlePtr,
    node_name1: *const c_char,
    node_name2: *const c_char,
) -> f32 {
    return distance_to_other_impl(ptr, node_name1, node_name2);
}

// ------------------------------------- Reingold Tilford Tree Layout -------------------------------------

#[no_mangle]
pub extern "C" fn draw_hierarchy(ptr: InHandlePtr, node_visitor: CBFnNodeVisitor) -> FFIError {
    return draw_hierarchy_impl(ptr, node_visitor);
}

#[no_mangle]
pub unsafe extern "C" fn draw_hierarchy_offset_from(
    ptr: InHandlePtr,
    root: Option<&ClusterData>,
    current_depth: i32,
    max_depth: i32,
    node_visitor: CBFnNodeVisitor,
) -> FFIError {
    return draw_hierarchy_offset_from_impl(ptr, root, current_depth, max_depth, node_visitor);
}

// ------------------------------------- Graph Physics -------------------------------------

#[no_mangle]
pub unsafe extern "C" fn init_force_directed_graph(
    context: InHandlePtr,
    arr_ptr: *mut ClusterData,
    len: i32,
    scalar: f32,
    max_iters: i32,
    // edge_detect_cb: CBFnNodeVisitorMut,
    // physics_update_cb: CBFnNodeVisitor,
) -> FFIError {
    return init_force_directed_graph_impl(
        context, arr_ptr, len, scalar, max_iters,
        // edge_detect_cb,
    );
}

#[no_mangle]
pub unsafe extern "C" fn init_graph_vertices(
    context: InHandlePtr,
    edge_detect_cb: CBFnNodeVisitorMut,
) -> FFIError {
    return init_graph_vertices_impl(context, edge_detect_cb);
}

#[no_mangle]
pub unsafe extern "C" fn physics_update_async(
    context: InHandlePtr,
    updater: CBFnNodeVisitor,
) -> FFIError {
    return physics_update_async_impl(context, updater);
}

#[no_mangle]
pub extern "C" fn shutdown_physics(ptr: InHandlePtr) -> FFIError {
    return shutdown_physics_impl(ptr);
}

#[no_mangle]
pub extern "C" fn get_num_edges_in_graph(ptr: InHandlePtr) -> i32 {
    return get_num_edges_in_graph_impl(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn force_physics_shutdown(ptr: InHandlePtr) -> i32 {
    // Handle::from_ptr(ptr).get_num_nodes() + 1
    // force_physics_shutdown_impl(ptr);
    if let Some(handle) = ptr {
        // debug!("cardinality: {}", handle.tree_height() + 1);
        handle.force_physics_shutdown();
        return 0;
        // return handle.tree_height() + 1;
    }
    debug!("handle not created force physics shutdown");

    return 0;
}
// ------------------------------------- RNN Search -------------------------------------

// #[no_mangle]
// pub unsafe extern "C" fn test_cakes_rnn_query(
//     ptr: InHandlePtr,
//     search_radius: f32,
//     node_visitor: CBFnNodeVisitor,
// ) -> FFIError {
//     return test_cakes_rnn_query_impl(ptr, search_radius, node_visitor);
// }

#[no_mangle]
pub unsafe extern "C" fn color_by_dist_to_query(
    context: InHandlePtr,
    arr_ptr: *mut ClusterData,
    len: i32,
    node_visitor: CBFnNodeVisitor,
) -> FFIError {
    return color_by_dist_to_query_impl(context, arr_ptr, len, node_visitor);
}
