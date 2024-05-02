/// Function that returns the cluster label of a cluster in the handle
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
/// The cluster label as an `i32` or -1 if the handle is not created
pub unsafe fn get_cluster_label_impl(ptr: InHandlePtr, cluster_id: *const c_char) -> i32 {
    // If the handle and label exist
    if let Some(handle) = ptr {
        if let Some(labels) = handle.labels() {
            // Get the cluster id as a string
            let cluster_id = helpers::c_char_to_string(cluster_id);
            // If the cluster exists in the handle, get the cluster label
            if let Ok(cluster) = handle.get_cluster_from_string(cluster_id) {
                let num_unique_labels = {
                    let unique_labels: HashSet<_> = labels.iter().cloned().collect();
                    unique_labels.len()
                };
                // Get the label colors
                let colors = helpers::label_colors();
                if num_unique_labels > colors.len() {
                    return -1;
                }
                // Get the dominant label of the cluster and return it or -1 if it doesn't exist
                match calc_cluster_dominant_label(cluster, labels, num_unique_labels, &colors) {
                    Some(label) => {
                        return label as i32;
                    }
                    None => {
                        return -1;
                    }
                }
                // if let Ok(degree) = clam_graph.vertex_degree(cluster) {
                //     return degree as i32;
                // }
            }
        }
    }
    debug!("handle not created");
    -1
}

/// Function that calculates the dominant label of a cluster
/// 
/// # Arguments
/// 
/// * `cluster` - The cluster to calculate the dominant label of
/// * `labels` - The labels of the cluster
/// * `num_unique_labels` - The number of unique labels
/// * `color_choices` - The color choices
/// 
/// # Returns
/// 
/// An `Option` containing the dominant label as a `usize`
fn calc_cluster_dominant_label(
    cluster: &Clusterf32,
    labels: &[u8],
    num_unique_labels: usize,
    color_choices: &Vec<glam::Vec3>,
) -> Option<usize> {
    let indices = cluster.indices();
    // let unique_values: HashSet<_> = labels.iter().cloned().collect();

    // set up a vector to store the entropy
    let mut entropy = vec![0; num_unique_labels];
    indices.for_each(|i| entropy[labels[i] as usize] += 1);
    // iterate through the entropy and get the max index, which is the dominant label
    let max_index = entropy
        .iter()
        .enumerate()
        .max_by_key(|&(_, val)| val)
        .map(|(index, _)| index);

    max_index

    // glam::Vec3::new(perc_outliers, perc_inliers, 0.)
}