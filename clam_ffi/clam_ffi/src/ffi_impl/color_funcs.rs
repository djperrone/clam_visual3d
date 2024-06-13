/// Function that colors a provided cluster by the entropy of the cluster
/// 
/// # Arguments
/// 
/// * `ptr` - A pointer to the handle
/// * `node_visitor` - A function pointer to the node visitor function
/// 
/// # Returns
/// 
/// An `FFIError` enum
pub fn color_clusters_by_entropy_impl(ptr: InHandlePtr, node_visitor: CBFnNodeVisitor) -> FFIError {
    // If the handle and root exist
    if let Some(handle) = ptr {
        if let Some(root) = handle.root() {
            // If the labels exist
            if let Some(labels) = handle.labels() {
                // Color the clusters by entropy
                color_helper(Some(root), labels, node_visitor);
                return FFIError::Ok;
            }
        }
    }
    // Return an error if the handle is not created
    FFIError::HandleInitFailed
}

/// Function that calculates the entropy color of a cluster
/// 
/// # Arguments
/// 
/// * `cluster` - The cluster to calculate the entropy color of
/// * `labels` - The labels of the cluster
/// 
/// # Returns
/// 
/// A `glam::Vec3` representing the entropy color
fn calc_cluster_entropy_color(cluster: &Clusterf32, labels: &[u8]) -> glam::Vec3 {
    // Get the indices of the cluster
    let indices = cluster.indices();
    let mut entropy = [0; 2];
    // Calculate the entropy of the cluster
    indices.for_each(|i| entropy[labels[i] as usize] += 1);

    // Calculate the total entropy
    let total_entropy: u32 = entropy.iter().sum();

    // Calculate the percentage of inliers and outliers
    let perc_inliers = entropy[0] as f32 / total_entropy as f32;
    let perc_outliers = entropy[1] as f32 / total_entropy as f32;

    // Return the entropy color
    glam::Vec3::new(perc_outliers, perc_inliers, 0.)
}

/// Function that calculates the dominant color of a cluster
/// 
/// # Arguments
/// 
/// * `cluster` - The cluster to calculate the dominant color of
/// * `labels` - The labels of the cluster
/// * `num_unique_labels` - The number of unique labels
/// * `color_choices` - The color choices
/// 
/// # Returns
/// 
/// A `Result` containing the dominant color as a `glam::Vec3` or an error message as a `String`
fn calc_cluster_dominant_color(
    cluster: &Clusterf32,
    labels: &[u8],
    num_unique_labels: usize,
    color_choices: &Vec<glam::Vec3>,
) -> Result<glam::Vec3, String> {
    // Calculate the dominant label of the cluster
    let max_index = calc_cluster_dominant_label(cluster, labels, num_unique_labels, color_choices);

    // Return the dominant color or an error message
    match max_index {
        Some(dom_label) => Ok(color_choices[dom_label as usize]),
        None => Err("invalid labels? i guess".to_string()),
    }

    // glam::Vec3::new(perc_outliers, perc_inliers, 0.)
}

/// Function that colors the clusters by the dominant label of the cluster
/// 
/// # Arguments
/// 
/// * `ptr` - A pointer to the handle
/// * `node_visitor` - A function pointer to the node visitor function
fn color_helper(root: Option<&Clusterf32>, labels: &[u8], node_visitor: CBFnNodeVisitor) {
    // If the root exists
    if let Some(cluster) = root {
        // Get the cluster data
        let mut cluster_data = ClusterDataWrapper::from_cluster(cluster);
        cluster_data.data_mut().color = calc_cluster_entropy_color(cluster, labels);
        
        // Visit the node
        node_visitor(Some(cluster_data.data()));

        // If the cluster has children, color the children
        if let Some([left, right]) = cluster.children() {
            color_helper(Some(left), labels, node_visitor);
            color_helper(Some(right), labels, node_visitor);
        }
    }
}

/// Function that colors the clusters by the dominant label of the cluster
/// 
/// # Arguments
/// 
/// * `ptr` - A pointer to the handle
/// * `node_visitor` - A function pointer to the node visitor function
/// 
/// # Returns
/// 
/// An `FFIError` enum
pub fn color_clusters_by_dominant_label_impl(
    ptr: InHandlePtr,
    node_visitor: CBFnNodeVisitor,
) -> FFIError {
    // If the handle and root exist
    if let Some(handle) = ptr {
        if let Some(root) = handle.root() {
            // If the labels exist
            if let Some(labels) = handle.labels() {
                // Color the clusters by the dominant label
                let num_unique_labels = {
                    let unique_labels: HashSet<_> = labels.iter().cloned().collect();
                    unique_labels.len()
                };
                // Get the label colors
                let colors = helpers::label_colors();
                if num_unique_labels > colors.len() {
                    return FFIError::TooManyLabels;
                }
                // Color the clusters by the dominant label
                for c in root.subtree() {
                    let mut cluster_data = ClusterDataWrapper::from_cluster(c);
                    if let Ok(color) =
                        calc_cluster_dominant_color(c, labels, num_unique_labels, &colors)
                    {
                        cluster_data.data_mut().color = color;
                        node_visitor(Some(cluster_data.data()));
                    } else {
                        return FFIError::ColoringFailed;
                    }
                }
                return FFIError::Ok;
            }
        }
    }

    FFIError::HandleInitFailed
}

/// Function that colors the clusters by the distance to the query
/// 
/// # Safety
/// 
/// This function is unsafe because it dereferences the pointer passed to it
/// 
/// # Arguments
/// 
/// * `context` - A pointer to the handle
/// * `arr_ptr` - A pointer to the cluster data
/// * `len` - The length of the cluster data
/// * `node_visitor` - A function pointer to the node visitor function
/// 
/// # Returns
/// 
/// An `FFIError` enum
pub unsafe fn color_by_dist_to_query_impl(
    context: InHandlePtr,
    arr_ptr: *mut ClusterData,
    len: i32,
    node_visitor: CBFnNodeVisitor,
) -> FFIError {
    // If the handle and cluster data exist
    if let Some(handle) = context {
        if arr_ptr.is_null() {
            return FFIError::NullPointerPassed;
        }
        // Get the cluster data from the pointer
        let arr = std::slice::from_raw_parts(arr_ptr, len as usize);

        let mut ids = Vec::new();

        // Get the ids of the clusters
        for node in arr {
            ids.push(node.id.as_string().unwrap());
        }

        // Color the clusters by the distance to the query
        handle.color_by_dist_to_query(ids.as_slice(), node_visitor)
    } else {
        FFIError::NullPointerPassed
    }
}