use crate::{
    debug, ffi_impl::cluster_data::ClusterData, utils::{error::FFIError, types::InHandlePtr}, CBFnNameSetter, CBFnNodeVisitor
};

use super::graph_builder;

pub unsafe fn physics_update_async_impl(
    context: InHandlePtr,
    updater: CBFnNodeVisitor,
) -> FFIError {
    if let Some(handle) = context {
        handle.physics_update_async(updater)
    } else {
        FFIError::NullPointerPassed
    }
}

pub fn init_force_directed_graph_impl(
    context: InHandlePtr,
    scalar: f32,
    max_iters: i32,
) -> FFIError {
    if let Some(handle) = context {
        match graph_builder::build_force_directed_graph_async(handle, scalar, max_iters) {
            Ok(g) => {
                handle.set_graph(g);
                FFIError::Ok
            }
            Err(e) => {
                debug!("launch physics thread result {:?}", e);
                e
            }
        }
    } else {
        return FFIError::NullPointerPassed;
    }
}
pub unsafe fn init_graph_vertices_impl(
    context: InHandlePtr,
    edge_detect_cb: CBFnNameSetter,
) -> FFIError {
    if let Some(handle) = context {
        return handle.init_unity_edges(edge_detect_cb);
    }
    FFIError::Ok
}

// pub fn shutdown_physics_impl(ptr: InHandlePtr) -> FFIError {
//     if let Some(handle) = ptr {
//         return handle.shutdown_physics();
//     }
//     FFIError::NullPointerPassed
// }

pub fn get_num_edges_in_graph_impl(ptr: InHandlePtr) -> i32 {
    if let Some(handle) = ptr {
        debug!("num edges {}", handle.get_num_edges_in_graph());
        if let Some(graph) = handle.clam_graph() {
            return graph.edge_cardinality() as i32;
        }
    }
    -1
}

pub fn get_num_graph_components_impl(ptr: InHandlePtr) -> i32 {
    if let Some(handle) = ptr {
        if let Some(clam_graph) = handle.clam_graph() {
            return clam_graph.find_component_clusters().len() as i32;
        }
    }
    -1
}

pub fn get_graph_cluster_cardinality_impl(ptr: InHandlePtr) -> i32 {
    if let Some(handle) = ptr {
        if let Some(clam_graph) = handle.clam_graph() {
            return clam_graph.vertex_cardinality() as i32;
        }
    }
    -1
}
