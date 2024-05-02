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
    if let Some(handle) = ptr {
        let node1 = handle.get_cluster_from_string(helpers::c_char_to_string(node_name1));
        let node2 = handle.get_cluster_from_string(helpers::c_char_to_string(node_name2));

        // Return the distance to the other cluster or -1.0 if it doesn't exist
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