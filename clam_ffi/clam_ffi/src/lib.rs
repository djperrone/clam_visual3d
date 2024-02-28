use std::ffi::c_char;
mod ffi_impl;
mod file_io;
pub mod graph;
pub mod handle;
mod tests;
mod tree_layout;
mod utils;

use crate::ffi_impl::lib_impl::{
    free_resource, max_lfd_impl, max_vertex_degree_impl, vertex_degree_impl,
};
use crate::ffi_impl::tree_startup_data_ffi::TreeStartupDataFFI;
use crate::file_io::load_save::save_cakes_single_impl;
use ffi_impl::{
    cluster_data::ClusterData, cluster_ids::ClusterIDs, lib_impl::*, string_ffi::StringFFI,
};
use graph::entry::*;
use tests::test::run_triangle_test_impl;
use tree_layout::entry_point::{draw_hierarchy_impl, draw_hierarchy_offset_from_impl};
use utils::{
    debug,
    distances::DistanceMetric,
    error::FFIError,
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
        return match handle.get_cluster_from_string(id) {
            Ok(cluster) => {
                let cluster_data = ClusterData::from_clam(cluster);

                *outgoing = cluster_data;
                FFIError::Ok
            }
            Err(_) => FFIError::InvalidStringPassed,
        };
    }
    FFIError::NullPointerPassed
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
    FFIError::Ok
}

#[no_mangle]
pub extern "C" fn delete_cluster_data(
    in_cluster_data: Option<&ClusterData>,
    out_cluster_data: Option<&mut ClusterData>,
) -> FFIError {
    free_resource(in_cluster_data, out_cluster_data)
}

#[no_mangle]
pub unsafe extern "C" fn free_string(
    in_data: Option<&StringFFI>,
    out_data: Option<&mut StringFFI>,
) -> FFIError {
    free_resource(in_data, out_data)
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
        return match handle.get_cluster_from_string(id) {
            Ok(cluster) => {
                let cluster_data = ClusterIDs::from_clam(cluster);

                *outgoing = cluster_data;
                FFIError::Ok
            }
            Err(_) => FFIError::InvalidStringPassed,
        };
    }
    FFIError::NullPointerPassed
}

//noinspection ALL
#[no_mangle]
pub extern "C" fn delete_cluster_ids(
    in_cluster_data: Option<&ClusterIDs>,
    out_cluster_data: Option<&mut ClusterIDs>,
) -> FFIError {
    free_resource(in_cluster_data, out_cluster_data)
}

#[no_mangle]
pub unsafe extern "C" fn set_message(
    msg: *const c_char,
    out_cluster_data: Option<&mut ClusterData>,
) -> FFIError {
    if let Some(out_data) = out_cluster_data {
        let msg_str = StringFFI::c_char_to_string(msg);

        out_data.set_message(msg_str);
        FFIError::Ok
    } else {
        FFIError::NullPointerPassed
    }
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
    init_clam_impl(ptr, data_name, name_len, cardinality, distance_metric)
}

#[no_mangle]
pub unsafe extern "C" fn init_clam_struct(
    ptr: OutHandlePtr,
    data: Option<&TreeStartupDataFFI>,
) -> FFIError {
    init_clam_struct_impl(ptr, data)
}

#[no_mangle]
pub unsafe extern "C" fn load_cakes_struct(
    ptr: OutHandlePtr,
    data: Option<&TreeStartupDataFFI>,
) -> FFIError {
    load_cakes_struct_impl(ptr, data)
}

#[no_mangle]
pub unsafe extern "C" fn save_cakes(
    ptr: InHandlePtr,
    file_name: *const u8,
    name_len: i32,
) -> FFIError {
    save_cakes_single_impl(ptr, file_name, name_len)
}
#[no_mangle]
pub unsafe extern "C" fn shutdown_clam(context_ptr: OutHandlePtr) -> FFIError {
    shutdown_clam_impl(context_ptr)
}

// ------------------------------------- Graph Clam Init -------------------------------------
#[no_mangle]
pub extern "C" fn init_clam_graph(
    context: InHandlePtr,
    scoring_function: ScoringFunction,
    min_depth: i32,
    cluster_selector: CBFnNodeVisitor,
) -> FFIError {
    if let Some(handle) = context {
        return handle.init_clam_graph(scoring_function, min_depth, cluster_selector);
    }
    FFIError::HandleInitFailed
}

// -------------------------------------  Tree helpers -------------------------------------

#[no_mangle]
pub unsafe extern "C" fn for_each_dft(
    ptr: InHandlePtr,
    node_visitor: CBFnNodeVisitor,
    start_node: *const c_char,
    max_depth: i32,
) -> FFIError {
    for_each_dft_impl(ptr, node_visitor, start_node, max_depth)
}

#[no_mangle]
pub unsafe extern "C" fn set_names(
    ptr: InHandlePtr,
    node_visitor: CBFnNameSetter,
    start_node: *const c_char,
) -> FFIError {
    set_names_impl(ptr, node_visitor, start_node)
}

#[no_mangle]
pub unsafe extern "C" fn tree_height(ptr: InHandlePtr) -> i32 {
    tree_height_impl(ptr)
}

#[no_mangle]
pub unsafe extern "C" fn tree_cardinality(ptr: InHandlePtr) -> i32 {
    tree_cardinality_impl(ptr)
}

#[no_mangle]
pub unsafe extern "C" fn vertex_degree(ptr: InHandlePtr, cluster_id: *const c_char) -> i32 {
    vertex_degree_impl(ptr, cluster_id)
}

#[no_mangle]
pub unsafe extern "C" fn get_cluster_label(ptr: InHandlePtr, cluster_id: *const c_char) -> i32 {
    get_cluster_label_impl(ptr, cluster_id)
}

#[no_mangle]
pub unsafe extern "C" fn max_vertex_degree(ptr: InHandlePtr) -> i32 {
    max_vertex_degree_impl(ptr)
}

#[no_mangle]
pub unsafe extern "C" fn max_lfd(ptr: InHandlePtr) -> f32 {
    max_lfd_impl(ptr)
}

#[no_mangle]
// add recursive bool option and node name
pub unsafe extern "C" fn color_clusters_by_entropy(
    ptr: InHandlePtr,
    node_visitor: CBFnNodeVisitor,
) -> FFIError {
    ffi_impl::lib_impl::color_clusters_by_entropy_impl(ptr, node_visitor)
}

#[no_mangle]
pub unsafe extern "C" fn color_clusters_by_dominant_label(
    ptr: InHandlePtr,
    node_visitor: CBFnNodeVisitor,
) -> FFIError {
    ffi_impl::lib_impl::color_clusters_by_dominant_label_impl(ptr, node_visitor)
}
// ------------------------------------- Cluster Helpers -------------------------------------

#[no_mangle]
pub unsafe extern "C" fn distance_to_other(
    ptr: InHandlePtr,
    node_name1: *const c_char,
    node_name2: *const c_char,
) -> f32 {
    distance_to_other_impl(ptr, node_name1, node_name2)
}

// ------------------------------------- Reingold Tilford Tree Layout -------------------------------------

#[no_mangle]
pub extern "C" fn draw_hierarchy(ptr: InHandlePtr, node_visitor: CBFnNodeVisitor) -> FFIError {
    draw_hierarchy_impl(ptr, node_visitor)
}

#[no_mangle]
pub unsafe extern "C" fn draw_hierarchy_offset_from(
    ptr: InHandlePtr,
    root: Option<&ClusterData>,
    current_depth: i32,
    max_depth: i32,
    node_visitor: CBFnNodeVisitor,
) -> FFIError {
    draw_hierarchy_offset_from_impl(ptr, root, current_depth, max_depth, node_visitor)
}

// ------------------------------------- Graph Physics -------------------------------------

#[no_mangle]
pub extern "C" fn init_force_directed_graph(
    context: InHandlePtr,
    scalar: f32,
    max_iters: i32,
) -> FFIError {
    init_force_directed_graph_impl(context, scalar, max_iters)
}

#[no_mangle]
pub unsafe extern "C" fn init_graph_vertices(
    context: InHandlePtr,
    edge_detect_cb: CBFnNodeVisitorMut,
) -> FFIError {
    init_graph_vertices_impl(context, edge_detect_cb)
}

#[no_mangle]
pub unsafe extern "C" fn physics_update_async(
    context: InHandlePtr,
    updater: CBFnNodeVisitor,
) -> FFIError {
    physics_update_async_impl(context, updater)
}

#[no_mangle]
pub unsafe extern "C" fn run_triangle_test(
    context: InHandlePtr,
    last_run: bool,
    out_path: *const c_char,
    updater: CBFnNodeVisitorMut,
) -> FFIError {
    run_triangle_test_impl(context, 3, last_run, out_path, updater)
}

// #[no_mangle]
// pub extern "C" fn shutdown_physics(ptr: InHandlePtr) -> FFIError {
//     shutdown_physics_impl(ptr)
// }

#[no_mangle]
pub extern "C" fn get_num_edges_in_graph(ptr: InHandlePtr) -> i32 {
    get_num_edges_in_graph_impl(ptr)
}

#[no_mangle]
pub extern "C" fn get_graph_cluster_cardinality(ptr: InHandlePtr) -> i32 {
    get_graph_cluster_cardinality_impl(ptr)
}
#[no_mangle]
pub extern "C" fn get_num_graph_components(ptr: InHandlePtr) -> i32 {
    get_num_graph_components_impl(ptr)
}

#[no_mangle]
pub unsafe extern "C" fn force_physics_shutdown(ptr: InHandlePtr) -> i32 {
    if let Some(handle) = ptr {
        handle.force_physics_shutdown();
        return 0;
    }
    debug!("handle not created force physics shutdown");

    0
}
// ------------------------------------- RNN Search -------------------------------------
#[no_mangle]
pub unsafe extern "C" fn color_by_dist_to_query(
    context: InHandlePtr,
    arr_ptr: *mut ClusterData,
    len: i32,
    node_visitor: CBFnNodeVisitor,
) -> FFIError {
    color_by_dist_to_query_impl(context, arr_ptr, len, node_visitor)
}
