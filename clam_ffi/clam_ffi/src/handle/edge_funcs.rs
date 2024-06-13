/// Function to initialize the unity edges for the graph
/// 
/// # Arguments
/// 
/// * `self` - The handle
/// * `edge_detect_cb` - The edge detect callback function
/// 
/// # Returns
/// 
/// An `FFIError` indicating if the edges were initialized successfully or not
pub unsafe fn init_unity_edges(&mut self, edge_detect_cb: CBFnNodeVisitorMut) -> FFIError {
    if let Some(force_directed_graph) = &self.force_directed_graph {
        force_directed_graph::init_unity_edges(&force_directed_graph.1, edge_detect_cb);
    }
    FFIError::PhysicsAlreadyShutdown
}

/// Function to get the number of edges in the graph
/// 
/// # Arguments
/// 
/// * `self` - The handle
/// 
/// # Returns
/// 
/// An `i32` containing the number of edges in the graph or -1 if the graph does not exist
pub fn get_num_edges_in_graph(&self) -> i32 {
    if let Some(g) = self.clam_graph() {
        return g.edge_cardinality() as i32;
    }
    return -1;
}