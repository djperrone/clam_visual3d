/// Function to get the cluster name from a string
/// 
/// # Arguments
/// 
/// * `self` - The handle
/// * `cluster_id` - The cluster ID
/// 
/// # Returns
/// 
/// A `Result` containing a reference to the cluster or an `FFIError` if the cluster could not be found
// why isnt string taken by reference?
pub unsafe fn get_cluster_from_string(
    &self,
    cluster_id: String,
) -> Result<&Clusterf32, FFIError> {
    let mut parts = cluster_id.split('-');

    // If the cluster ID has an offset and a cardinality, get the cluster
    if let (Some(offset_str), Some(cardinality_str)) = (parts.next(), parts.next()) {
        if let (Ok(offset), Ok(cardinality)) = (
            offset_str.parse::<usize>(),
            cardinality_str.parse::<usize>(),
        ) {
            return self.get_cluster(offset, cardinality);
        }
    }
    debug!("root not built get_cluster from string");
    Err(FFIError::HandleInitFailed)
}

/// Function to get the cluster from an offset and a cardinality
/// 
/// # Arguments
/// 
/// * `self` - The handle
/// * `offset` - The offset of the cluster
/// * `cardinality` - The cardinality of the cluster
/// 
/// # Returns
/// 
/// A `Result` containing a reference to the cluster or an `FFIError` if the cluster could not be found
pub unsafe fn get_cluster(
    &self,
    offset: usize,
    cardinality: usize,
) -> Result<&Clusterf32, FFIError> {
    // If the tree exists, get the cluster from the offset and cardinality
    if let Some(tree) = self.tree() {
        return if let Some(cluster) = tree.get_cluster(offset, cardinality) {
            Ok(cluster)
        } else {
            Err(FFIError::InvalidStringPassed)
        };
    }
    debug!("root not built get cluster");
    Err(FFIError::HandleInitFailed)
}