/// Function to iterate through the tree in a depth-first traversal
/// 
/// # Arguments
/// 
/// * `self` - The handle
/// * `node_visitor` - The node visitor function
/// * `start_node` - The starting node
/// * `max_depth` - The maximum depth
/// 
/// # Returns
/// 
/// An `FFIError` indicating if the traversal was successful or not
pub unsafe fn for_each_dft(
    &self,
    node_visitor: CBFnNodeVisitor,
    start_node: String,
    max_depth: i32,
) -> FFIError {
    // If the tree exists
    return if self.tree().is_some() {
        if start_node == "root" {
            // If the start node is the root, iterate through the tree
            if let Some(node) = self.root() {
                Self::for_each_dft_helper(node, node_visitor, max_depth);
                FFIError::Ok
            } else {
                FFIError::HandleInitFailed
            }
        } else {
            // If the start node is not the root, get the cluster from the string
            match Self::get_cluster_from_string(self, start_node) {
                Ok(root) => {
                    // Iterate through the tree
                    Self::for_each_dft_helper(root, node_visitor, max_depth);
                    FFIError::Ok
                }
                Err(e) => {
                    debug!("{:?}", e);
                    FFIError::InvalidStringPassed
                }
            }
        }
    } else {
        FFIError::NullPointerPassed
    };
}

/// Helper function for depth-first traversal
/// 
/// # Arguments
/// 
/// * `root` - The root of the tree
/// * `node_visitor` - The node visitor function
/// * `max_depth` - The maximum depth
/// 
/// # Returns
/// 
/// Nothing
fn for_each_dft_helper(root: &Clusterf32, node_visitor: CBFnNodeVisitor, max_depth: i32) {
    // If the root is a leaf or the depth is greater than the maximum depth, set the node visitor
    if root.is_leaf() || root.depth() as i32 >= max_depth {
        let baton = ClusterDataWrapper::from_cluster(root);
        node_visitor(Some(baton.data()));
        return;
    }
    // If the root has children, set the node visitor and iterate through the children
    if let Some([left, right]) = root.children() {
        let baton = ClusterDataWrapper::from_cluster(root);

        node_visitor(Some(baton.data()));

        Self::for_each_dft_helper(left, node_visitor, max_depth);
        Self::for_each_dft_helper(right, node_visitor, max_depth);
    }
}