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
pub unsafe fn tree_cardinality_impl(ptr: InHandlePtr) -> i32 {
    if let Some(handle) = ptr {
        if let Some(tree) = handle.get_tree() {
            return tree.cardinality() as i32;
        }
    }
    debug!("handle not created");
    -1
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
        if handle.get_tree().is_some() {
            // If the clam graph exists
            if let Some(graph) = handle.clam_graph() {
                // Get the max vertex degree of the graph
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