/// Function to create a clam graph from the tree
/// 
/// # Arguments
/// 
/// * `self` - The handle
/// * `scoring_function` - The scoring function to use
/// * `min_depth` - The minimum depth of the graph
/// * `cluster_selector` - The cluster selector function
/// 
/// # Returns
/// 
/// An `FFIError` indicating if the graph was created successfully or not
pub fn init_clam_graph(
    &'a mut self,
    scoring_function: ScoringFunction,
    min_depth: i32,
    cluster_selector: CBFnNodeVisitor,
) -> FFIError {
    // If the tree exists
    if let Some(tree) = &self.tree {
        // Get the scoring function from the enum
        match enum_to_function(&scoring_function) {
            Ok(scorer) => {
                // Create the graph from the tree and the scoring function
                if let Ok(graph) = Graph::from_tree(tree, &scorer, min_depth as usize) {
                    self.clam_graph = Some(graph);
                    for cluster in self.clam_graph().unwrap().clusters() {
                        let baton = ClusterDataWrapper::from_cluster(cluster);
                        cluster_selector(Some(baton.data()));
                    }

                    return FFIError::Ok;
                }
            }
            Err(e) => {
                return e;
            }
        }
    }
    FFIError::GraphBuildFailed
}

/// Function to create a clam graph from the tree without a visual
/// 
/// # Arguments
/// 
/// * `self` - The handle
/// * `scoring_function` - The scoring function to use
/// * `min_depth` - The minimum depth of the graph
/// 
/// # Returns
/// 
/// An `FFIError` indicating if the graph was created successfully or not
pub fn init_clam_graph_no_visual(
    &'a mut self,
    scoring_function: ScoringFunction,
    min_depth: i32,
) -> FFIError {
    // If the tree exists
    if let Some(tree) = &self.tree {
        // Get the scoring function from the enum
        match enum_to_function(&scoring_function) {
            Ok(scorer) => {
                // Create the graph from the tree and the scoring function
                if let Ok(graph) = Graph::from_tree(tree, &scorer, min_depth as usize) {
                    self.clam_graph = Some(graph);

                    return FFIError::Ok;
                }
            }
            Err(e) => {
                return e;
            }
        }
    }
    FFIError::GraphBuildFailed
}

/// Function to set the graph
/// 
/// # Arguments
/// 
/// * `self` - The handle
/// * `graph` - The graph to set
pub fn set_graph(&mut self, graph: (JoinHandle<()>, Arc<ForceDirectedGraph>)) {
    self.force_directed_graph = Some(graph);
}

/// Function to get the clam graph of the handle
/// 
/// # Arguments
/// 
/// * `self` - The handle
/// 
/// # Returns
/// 
/// An `Option` containing a reference to the graph or `None` if the graph does not exist
pub fn clam_graph(&self) -> Option<&Graph<'a, f32>> {
    self.clam_graph.as_ref()
}