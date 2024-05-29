/// Function to create a reginald tilford layout. Runs the layout algorithm within the function.
/// 
/// # Arguments
/// 
/// * `self` - The handle
/// * `node_visitor` - The node visitor function
/// 
/// # Returns
/// 
/// An `FFIError` indicating if the layout was created successfully or not
pub fn create_reingold_layout(&self, node_visitor: CBFnNodeVisitor) -> FFIError {
    return if self.tree().is_some() {
        reingold_tilford::run(
            self.root()
                .unwrap_or_else(|| unreachable!("cakes exists - root should exist")),
            self.tree().unwrap().depth() as i32,
            node_visitor,
        )
    } else {
        FFIError::HandleInitFailed
    };
}

/// Function to create a reginald tilford layout with an offset. Runs the layout algorithm within the function.
/// 
/// # Arguments
/// 
/// * `self` - The handle
/// * `root` - The root of the cluster
/// * `current_depth` - The current depth
/// * `max_depth` - The maximum depth
/// * `node_visitor` - The node visitor function
/// 
/// # Returns
/// 
/// An `FFIError` indicating if the layout was created successfully or not
pub unsafe fn create_reingold_layout_offset_from(
    &self,
    root: &ClusterData,
    _current_depth: i32,
    max_depth: i32,
    node_visitor: CBFnNodeVisitor,
) -> FFIError {
    return if self.tree().is_some() {
        if let Ok(clam_root) = self.get_cluster_from_string(root.get_id()) {
            reingold_tilford::run_offset(&root.pos, clam_root, max_depth, node_visitor)
        } else {
            FFIError::NullPointerPassed
        }
    } else {
        FFIError::HandleInitFailed
    };
}