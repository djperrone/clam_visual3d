/// Function to force a shutdown of the graph physics
/// 
/// # Arguments
/// 
/// * `self` - The handle
/// 
/// # Returns
/// 
/// An `FFIError` indicating if the physics was shutdown successfully or not
pub unsafe fn force_physics_shutdown(&mut self) -> FFIError {
    // If the force directed graph exists, shutdown the physics
    if let Some(force_directed_graph) = &self.force_directed_graph {
        force_directed_graph::force_shutdown(&force_directed_graph.1);
        let _ = self.force_directed_graph.take().unwrap().0.join();

        self.force_directed_graph = None;
        debug!("force shutting down physics");
        return FFIError::PhysicsFinished;
    }
    FFIError::PhysicsAlreadyShutdown
}

/// Function to update the physics asynchronously
/// 
/// # Arguments
/// 
/// * `self` - The handle
/// * `updater` - The node visitor function
/// 
/// # Returns
/// 
/// An `FFIError` indicating if the physics was updated successfully or not
pub unsafe fn physics_update_async(&mut self, updater: CBFnNodeVisitor) -> FFIError {
    // If the force directed graph exists, update the physics
    if let Some(force_directed_graph) = &self.force_directed_graph {
        // If the physics is finished, join the thread and set the force directed graph to `None`
        let is_finished = force_directed_graph.0.is_finished();

        return if is_finished {
            let _ = self.force_directed_graph.take().unwrap().0.join();
            self.force_directed_graph = None;
            debug!("shutting down physics");

            FFIError::PhysicsFinished
        } else {
            force_directed_graph::try_update_unity(
                &force_directed_graph.1,
                self.clam_graph().as_ref().unwrap(),
                self.tree().as_ref().unwrap(),
                updater,
            )
        };
    }

    FFIError::PhysicsAlreadyShutdown
}