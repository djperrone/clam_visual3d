/// Function to set the names of the clusters
/// 
/// # Arguments
/// 
/// * `self` - The handle
/// * `node_visitor` - The node visitor function
/// * `start_node` - The starting node
/// 
/// # Returns
/// 
/// An `FFIError` indicating if the names were set successfully or not
pub unsafe fn set_names(
    &self,
    node_visitor: crate::CBFnNameSetter,
    start_node: String,
) -> FFIError {
    // If the tree exists
    return if self.tree().is_some() {
        // If the start node is the root, set the names of the clusters
        if start_node == "root" {
            if let Some(node) = self.root() {
                Self::set_names_helper(node, node_visitor);
                FFIError::Ok
            } else {
                FFIError::HandleInitFailed
            }
        } else {
            // If the start node is not the root, get the cluster from the string
            match Self::get_cluster_from_string(self, start_node) {
                Ok(root) => {
                    Self::set_names_helper(root, node_visitor);
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

/// Helper function for name definition
/// 
/// # Arguments
/// 
/// * `root` - The root of the tree
/// * `node_visitor` - The node visitor function
/// 
/// # Returns
/// 
/// Nothing
fn set_names_helper(root: &Clusterf32, node_visitor: crate::CBFnNameSetter) {
    // If the root is a leaf, set the name of the cluster
    if root.is_leaf() {
        let baton = ClusterIDsWrapper::from_cluster(root);

        node_visitor(Some(baton.data()));
        return;
    }
    // If the root has children, set the name of the cluster and iterate through the children
    if let Some([left, right]) = root.children() {
        let baton = ClusterIDsWrapper::from_cluster(root);

        node_visitor(Some(baton.data()));
        Self::set_names_helper(left, node_visitor);
        Self::set_names_helper(right, node_visitor);
    }
}