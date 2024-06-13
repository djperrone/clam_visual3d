/// Function to color the graph based on the distance to the query
/// 
/// # Arguments
/// 
/// * `self` - The handle
/// * `id_arr` - An array of strings containing the cluster IDs
/// * `node_visitor` - The node visitor function
pub unsafe fn color_by_dist_to_query(
    &self,
    id_arr: &[String],
    node_visitor: CBFnNodeVisitor,
) -> FFIError {
    // If the current query exists
    for id in id_arr {
        match self.get_cluster_from_string(id.clone()) {
            Ok(cluster) => {
                if let Some(query) = &self.current_query {
                    // Color the cluster by the distance to the query
                    let mut baton_data = ClusterDataWrapper::from_cluster(cluster);

                    baton_data.data_mut().dist_to_query =
                        cluster.distance_to_instance(self.data().unwrap(), query);

                    node_visitor(Some(baton_data.data()));
                } else {
                    return FFIError::QueryIsNull;
                }
            }
            Err(e) => {
                return e;
            }
        }
    }
    FFIError::Ok
}

/// Function to set the current query
/// 
/// # Arguments
/// 
/// * `self` - The handle
/// * `data` - A reference to the query data
/// 
/// # Returns
/// 
/// Nothing
pub fn set_current_query(&mut self, _data: &Vec<f32>) {
    todo!()
    // self.current_query = Some(data.clone());
}

/// Function to get the current query
/// 
/// # Arguments
/// 
/// * `self` - The handle
/// 
/// # Returns
/// 
/// An `Option` containing the current query or `None` if the query does not exist
pub fn get_current_query(&self) -> &Option<Vec<f32>> {
    &self.current_query
}

// pub fn rnn_search(
//     &self,
//     query: &Vec<f32>,
//     radius: f32,
// ) -> Result<(Vec<(&Clusterf32, f32)>, Vec<(&Clusterf32, f32)>), FFIError> {
//     if let Some(cakes) = &self.cakes {
//         // temporary fix later
//         // self.current_query = Some(query.clone());
//         return Ok(cakes.rnn_search_candidates(query, radius));
//     }
//     return Err(FFIError::NullPointerPassed);
// }