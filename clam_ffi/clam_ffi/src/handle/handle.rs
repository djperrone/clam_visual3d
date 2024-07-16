extern crate nalgebra as na;

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use abd_clam::graph::Graph;
use abd_clam::Cluster;
use abd_clam::Dataset;
// use abd_clam::criteria::detect_edges;
use abd_clam::Tree;
use abd_clam::VecDataset;
use abd_clam::{graph, PartitionCriteria};

use crate::ffi_impl::cluster_ids::ClusterID;
use crate::ffi_impl::cluster_ids::ClusterIDs;
// use crate::ffi_impl::cluster_ids_wrapper::ClusterIDsWrapper;
// use crate::graph;
use crate::graph::force_directed_graph::{self, ForceDirectedGraph};
use crate::graph::spring;
use crate::tree_layout::reingold_tilford;
use crate::utils::distances::DistanceMetric;
use crate::utils::error::FFIError;
use crate::utils::scoring_functions::enum_to_function;
use crate::utils::types::Graphf32;
use crate::utils::types::Treef32;
use crate::utils::types::{DataSetf32, Vertexf32};
use crate::utils::{self, anomaly_readers};

use crate::CBFnNameSetter;
use crate::{debug, CBFnNodeVisitor, CBFnNodeVisitorMut};

use crate::ffi_impl::cluster_data::ClusterData;
// use crate::ffi_impl::cluster_data_wrapper::ClusterDataWrapper;
use crate::ffi_impl::tree_startup_data_ffi::TreeStartupDataFFI;
use crate::graph::physics_node::PhysicsNode;
use crate::utils::scoring_functions::ScoringFunction;
use spring::Spring;

pub struct Handle<'a> {
    tree: Option<Treef32>,
    clam_graph: Option<Graphf32<'a>>,
    edges: Option<Vec<Spring>>,
    current_query: Option<Vec<f32>>,
    force_directed_graph: Option<(JoinHandle<()>, Arc<ForceDirectedGraph>)>,
}
impl<'a> Handle<'a> {
    // pub fn from(
    //     tree: Tree<Vec<f32>, f32, DataSetf32>,
    //     clam_graph: Graphf32,
    //     force_directed_graph: ForceDirectedGraph,
    // ) {
    //     let force_directed_graph = Arc::new(force_directed_graph);

    //     let b = force_directed_graph.clone();
    //     let p = thread::spawn(move || {
    //         graph::force_directed_graph::produce_computations(&b);
    //     });
    //     Handle {
    //         tree: Some(tree),
    //         clam_graph: Some(clam_graph),
    //         force_directed_graph: Some(p),
    //     }
    // }

    /// Function to shutdown the handle
    ///
    /// This function sets the tree to `None` and the labels to `None`
    ///
    /// # Arguments
    ///
    /// * `self` - The handle
    ///
    /// # Returns
    ///
    /// Nothing
    pub fn shutdown(&mut self) {
        self.tree = None;
        // self.labels = None;
    }

    /// Function to get the tree
    ///
    /// # Arguments
    ///
    /// * `self` - The handle
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the tree or `None` if the tree does not exist
    pub fn tree(&self) -> Option<&Treef32> {
        self.tree.as_ref()
    }

    /// Function to get the data
    ///
    /// # Arguments
    ///
    /// * `self` - The handle
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the data or `None` if the tree does not exist
    pub fn data(&self) -> Option<&DataSetf32> {
        // If the tree exists, return the data
        return if let Some(tree) = self.tree() {
            Some(tree.data())
        } else {
            None
        };
    }

    /// Function to get the root of the tree
    ///
    /// # Arguments
    ///
    /// * `self` - The handle
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the root of the tree or `None` if the tree does not exist
    pub fn root(&self) -> Option<&Vertexf32> {
        return if let Some(t) = self.tree() {
            Some(t.root())
        } else {
            None
        };
    }

    /// Function to get the labels of the tree
    ///
    /// # Arguments
    ///
    /// * `self` - The handle
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the labels or `None` if the tree does not exist
    pub fn labels(&self) -> Option<&[u8]> {
        match self.tree() {
            Some(tree) => Some(tree.data().metadata()),
            None => None,
        }
    }

    /// Function to create a new handle
    ///
    /// # Arguments
    ///
    /// * `data_name` - A string slice containing the name of the data
    /// * `cardinality` - The cardinality of the data
    /// * `distance_metric` - The distance metric to use
    /// * `is_expensive` - A boolean indicating if the distance metric is expensive
    ///
    /// # Returns
    ///
    /// A `Result` containing the handle or an `FFIError` if the handle could not be created
    pub fn new(
        data_name: &str,
        cardinality: usize,
        distance_metric: DistanceMetric,
        is_expensive: bool,
    ) -> Result<Self, FFIError> {
        // Get the data directory
        let mut data_dir = std::env::current_dir().unwrap();
        // println!(
        //     "data dir here 123 {}",
        //     data_dir.file_name().unwrap().to_str().unwrap()
        // );
        // Pop the current directory and add the data directory
        data_dir.pop();
        data_dir.push("data");
        data_dir.push("anomaly_data");
        data_dir.push("preprocessed");

        // Create the partition criteria with the cardinality
        let criteria = PartitionCriteria::new(true).with_min_cardinality(cardinality);

        // Create the dataset from the data name, data directory, distance metric, and if the distance metric is expensive
        match Self::create_dataset(data_name, &data_dir, distance_metric, is_expensive) {
            // If the dataset was created successfully, create the tree from the dataset and partition it with the criteria
            Ok(dataset) => {
                let tree = Tree::new(dataset, Some(1)).partition(&criteria, None);
                // Return the handle with the tree
                Ok(Handle {
                    tree: Some(tree),
                    clam_graph: None,
                    edges: None,
                    current_query: None,
                    force_directed_graph: None,
                })
            }
            // If the dataset could not be created, return an error
            Err(_) => Err(FFIError::HandleInitFailed),
        }
    }

    /// Function to load a handle from a tree startup data FFI struct
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to the tree startup data FFI struct
    ///
    /// # Returns
    ///
    /// A `Result` containing the handle or an `FFIError` if the handle could not be loaded
    pub fn load_struct(data: &TreeStartupDataFFI) -> Result<Self, FFIError> {
        // Get the data name from the data struct
        let data_name = match data.data_name.as_string() {
            Ok(data_name) => data_name,
            Err(e) => {
                debug!("{:?}", e);
                return Err(FFIError::InvalidStringPassed);
            }
        };

        // Get the distance metric from the data struct
        let metric = match utils::distances::from_enum(data.distance_metric) {
            Ok(metric) => metric,
            Err(e) => {
                debug!("{:?}", e);
                return Err(e);
            }
        };

        // Load the tree from the data name, distance metric, and if the distance metric is expensive
        if let Ok(tree) = Treef32::load(Path::new(&data_name), metric, data.is_expensive) {
            // if let Ok(tree) = Tree::<Vec<f32>, f32, DataSetf32>::load(
            //     Path::new(&data_name),
            //     metric,
            //     data.is_expensive,
            // ) {
            // let tree = tree.with_ratios(false);
            Ok(Handle {
                tree: Some(tree),
                clam_graph: None,
                edges: None,
                current_query: None,
                force_directed_graph: None,
            })
        } else {
            Err(FFIError::LoadTreeFailed)
        }
    }

    /// Function to create a dataset
    ///
    /// # Arguments
    ///
    /// * `data_name` - A string slice containing the name of the data
    /// * `data_dir` - A reference to the data directory
    /// * `distance_metric` - The distance metric to use
    /// * `is_expensive` - A boolean indicating if the distance metric is expensive
    ///
    /// # Returns
    ///
    /// A `Result` containing the dataset or an `FFIError` if the dataset could not be created
    pub fn create_dataset(
        data_name: &str,
        data_dir: &PathBuf,
        distance_metric: DistanceMetric,
        is_expensive: bool,
    ) -> Result<DataSetf32, FFIError> {
        // Get the distance metric from the enum
        let metric = match utils::distances::from_enum(distance_metric) {
            Ok(metric) => metric,
            Err(e) => {
                debug!("{:?}", e);
                return Err(e);
            }
        };

        // Read the anomaly data from the data name and data directory
        match anomaly_readers::read_anomaly_data(data_name, data_dir, false) {
            Ok((first_data, labels)) => {
                // let labels = labels.iter().map(|x| *x == 1).collect::<Vec<bool>>();
                let dataset =
                    VecDataset::new(data_name.to_string(), first_data, metric, is_expensive)
                        .assign_metadata(labels);

                // Return the dataset if it was created successfully or an error if it was not
                if dataset.is_ok() {
                    Ok(dataset.unwrap())
                } else {
                    return Err(FFIError::HandleInitFailed);
                }
            }
            Err(e) => {
                debug!("{:?}", e);
                Err(e)
            }
        }
    }

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
                        for cluster in self.clam_graph().unwrap().ordered_clusters() {
                            let data = ClusterData::from_clam(cluster);
                            cluster_selector(Some(&data));
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
    pub unsafe fn init_unity_edges(&mut self, edge_detect_cb: CBFnNameSetter) -> FFIError {
        if let Some(force_directed_graph) = &self.force_directed_graph {
            force_directed_graph::init_unity_edges(&force_directed_graph.1, edge_detect_cb);
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

    /// Function to set the graph
    ///
    /// # Arguments
    ///
    /// * `self` - The handle
    /// * `graph` - The graph to set
    pub fn set_graph(&mut self, graph: (JoinHandle<()>, Arc<ForceDirectedGraph>)) {
        self.force_directed_graph = Some(graph);
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
        // for id in id_arr {
        //     match self.get_cluster_from_string(id.clone()) {
        //         Ok(cluster) => {
        //             if let Some(query) = &self.current_query {
        //                 // Color the cluster by the distance to the query
        //                 let mut baton_data = ClusterDataWrapper::from_cluster(cluster);

        //                 baton_data.data_mut().dist_to_query =
        //                     cluster.distance_to_instance(self.data().unwrap(), query);

        //                 node_visitor(Some(baton_data.data()));
        //             } else {
        //                 return FFIError::QueryIsNull;
        //             }
        //         }
        //         Err(e) => {
        //             return e;
        //         }
        //     }
        // }
        FFIError::ColoringFailed
    }

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
        offset: usize,
        cardinality: usize,
        max_depth: i32,
    ) -> FFIError {
        debug!("fir eacgh dft");
        if let Some(tree) = self.tree(){
           if let Some(start_cluster) = tree.get_cluster(offset, cardinality){
                    Self::for_each_dft_helper(start_cluster, node_visitor, max_depth);
                    return  FFIError::Ok;
           }
        }
        return FFIError::NullPointerPassed;
        // If the tree exists
        // return if self.tree().is_some() {
        //     if offset == 0 && cardinality == tree {
        //         // If the start node is the root, iterate through the tree
        //         if let Some(node) = self.root() {
        //             Self::for_each_dft_helper(node, node_visitor, max_depth);
        //             FFIError::Ok
        //         } else {
        //             FFIError::HandleInitFailed
        //         }
        //     } else {
        //         // If the start node is not the root, get the cluster from the string
        //         match Self::get_cluster_from_string(self, start_node) {
        //             Ok(root) => {
        //                 // Iterate through the tree
        //                 Self::for_each_dft_helper(root, node_visitor, max_depth);
        //                 FFIError::Ok
        //             }
        //             Err(e) => {
        //                 debug!("{:?}", e);
        //                 FFIError::InvalidStringPassed
        //             }
        //         }
        //     }
        // } else {
        //     FFIError::NullPointerPassed
        // };
    }

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
        offset : usize,
        cardinality: usize
    ) -> FFIError {

        if let Some(_) = self.tree(){
            if let Ok(start_cluster) = self.get_cluster(offset, cardinality){
                Self::set_names_helper(start_cluster, node_visitor);
                return FFIError::Ok;
            }

        }
        return FFIError::NullPointerPassed;
        // If the tree exists
        // return if self.tree().is_some() {
        //     // If the start node is the root, set the names of the clusters
        //     if start_node == "root" {
        //         if let Some(node) = self.root() {
        //             Self::set_names_helper(node, node_visitor);
        //             FFIError::Ok
        //         } else {
        //             FFIError::HandleInitFailed
        //         }
        //     } else {
        //         // If the start node is not the root, get the cluster from the string
        //         match Self::get_cluster_from_string(self, start_node) {
        //             Ok(root) => {
        //                 Self::set_names_helper(root, node_visitor);
        //                 FFIError::Ok
        //             }
        //             Err(e) => {
        //                 debug!("{:?}", e);
        //                 FFIError::InvalidStringPassed
        //             }
        //         }
        //     }
        // } else {
        //     FFIError::NullPointerPassed
        // };
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
    fn set_names_helper(root: &Vertexf32, node_visitor: crate::CBFnNameSetter) {
        // If the root is a leaf, set the name of the cluster
        if root.is_leaf() {
            let data = ClusterIDs::from_clam(root);

            node_visitor(Some(&data));
            return;
        }
        // If the root has children, set the name of the cluster and iterate through the children
        if let Some([left, right]) = root.children() {
            let data = ClusterIDs::from_clam(root);

            node_visitor(Some(&data));
            Self::set_names_helper(left, node_visitor);
            Self::set_names_helper(right, node_visitor);
        }
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
    fn for_each_dft_helper(root: &Vertexf32, node_visitor: CBFnNodeVisitor, max_depth: i32) {
        // If the root is a leaf or the depth is greater than the maximum depth, set the node visitor
        if root.is_leaf() || root.depth() as i32 >= max_depth {
            let data = ClusterData::from_clam(root);
            node_visitor(Some(&data));
            return;
        }
        // If the root has children, set the node visitor and iterate through the children
        if let Some([left, right]) = root.children() {
            let data = ClusterData::from_clam(root);

            node_visitor(Some(&data));

            Self::for_each_dft_helper(left, node_visitor, max_depth);
            Self::for_each_dft_helper(right, node_visitor, max_depth);
        }
    }

    // pub fn shutdown_physics(&mut self) -> FFIError {
    //     let should_shutdown = { self.graph.is_some() && self.edges.is_some() };

    //     if should_shutdown {
    //         self.graph = None;
    //         self.edges = None;
    //         FFIError::Ok
    //     } else {
    //         FFIError::PhysicsAlreadyShutdown
    //     }
    // }

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
    // ) -> Result<(Vec<(&Vertexf32, f32)>, Vec<(&Vertexf32, f32)>), FFIError> {
    //     if let Some(cakes) = &self.cakes {
    //         // temporary fix later
    //         // self.current_query = Some(query.clone());
    //         return Ok(cakes.rnn_search_candidates(query, radius));
    //     }
    //     return Err(FFIError::NullPointerPassed);
    // }

    /// Function to get the number of nodes in the tree
    ///
    /// # Arguments
    ///
    /// * `self` - The handle
    ///
    /// # Returns
    ///
    /// An `i32` containing the number of nodes in the tree or 0 if the tree does not exist
    pub fn get_num_nodes(&self) -> i32 {
        if let Some(tree) = self.tree() {
            tree.cardinality() as i32
        } else {
            0
        }
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

    /// Function to get the tree height of the handle
    ///
    /// # Arguments
    ///
    /// * `self` - The handle
    ///
    /// # Returns
    ///
    /// An `i32` containing the height of the tree or 0 if the tree does not exist
    pub fn tree_height(&self) -> i32 {
        if let Some(tree) = self.tree() {
            tree.depth() as i32
        } else {
            0
        }
    }

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
    // pub unsafe fn get_cluster_from_string(
    //     &self,
    //     cluster_id: String,
    // ) -> Result<&Vertexf32, FFIError> {
    //     let mut parts = cluster_id.split('-');

    //     // If the cluster ID has an offset and a cardinality, get the cluster
    //     if let (Some(offset_str), Some(cardinality_str)) = (parts.next(), parts.next()) {
    //         if let (Ok(offset), Ok(cardinality)) = (
    //             offset_str.parse::<usize>(),
    //             cardinality_str.parse::<usize>(),
    //         ) {
    //             return self.get_cluster(offset, cardinality);
    //         }
    //     }
    //     debug!("root not built get_cluster from string");
    //     Err(FFIError::HandleInitFailed)
    // }

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
    ) -> Result<&Vertexf32, FFIError> {
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
            if let Ok(clam_root) = self.get_cluster(root.offset as usize, root.cardinality as usize) {
                reingold_tilford::run_offset(&root.pos, clam_root, max_depth, node_visitor)
            } else {
                FFIError::NullPointerPassed
            }
        } else {
            FFIError::HandleInitFailed
        };
    }
}
