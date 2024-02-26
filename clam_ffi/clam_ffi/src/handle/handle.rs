extern crate nalgebra as na;

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use abd_clam::Dataset;
// use abd_clam::criteria::detect_edges;
use abd_clam::Tree;
use abd_clam::VecDataset;
use abd_clam::{Graph, PartitionCriteria};

use crate::ffi_impl::cluster_ids_wrapper::ClusterIDsWrapper;
use crate::graph;
use crate::graph::force_directed_graph::{self, ForceDirectedGraph};
use crate::graph::spring;
use crate::tree_layout::reingold_tilford;
use crate::utils::distances::DistanceMetric;
use crate::utils::error::FFIError;
use crate::utils::scoring_functions::enum_to_function;
use crate::utils::types::Graphf32;
use crate::utils::types::{Clusterf32, DataSetf32};
use crate::utils::{self, anomaly_readers};

use crate::{debug, CBFnNodeVisitor, CBFnNodeVisitorMut};

use crate::ffi_impl::cluster_data::ClusterData;
use crate::ffi_impl::cluster_data_wrapper::ClusterDataWrapper;
use crate::ffi_impl::tree_startup_data_ffi::TreeStartupDataFFI;
use crate::graph::physics_node::PhysicsNode;
use crate::utils::scoring_functions::ScoringFunction;
use spring::Spring;

pub struct Handle<'a> {
    tree: Option<Tree<Vec<f32>, f32, DataSetf32>>,
    clam_graph: Option<Graph<'a, f32>>,
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

    pub fn shutdown(&mut self) {
        self.tree = None;
        // self.labels = None;
    }

    pub fn get_tree(&self) -> Option<&Tree<Vec<f32>, f32, VecDataset<Vec<f32>, f32, u8>>> {
        self.tree.as_ref()
    }

    pub fn tree(&self) -> Option<&Tree<Vec<f32>, f32, VecDataset<Vec<f32>, f32, u8>>> {
        self.tree.as_ref()
    }

    pub fn data(&self) -> Option<&DataSetf32> {
        return if let Some(tree) = self.tree() {
            Some(tree.data())
        } else {
            None
        };
    }
    pub fn root(&self) -> Option<&Clusterf32> {
        return if let Some(t) = self.tree() {
            Some(t.root())
        } else {
            None
        };
    }

    pub fn labels(&self) -> Option<&[u8]> {
        match self.tree() {
            Some(tree) => Some(tree.data().metadata()),
            None => None,
        }
    }

    pub fn new(
        data_name: &str,
        cardinality: usize,
        distance_metric: DistanceMetric,
        is_expensive: bool,
    ) -> Result<Self, FFIError> {
        let mut data_dir = std::env::current_dir().unwrap();
        // println!(
        //     "data dir here 123 {}",
        //     data_dir.file_name().unwrap().to_str().unwrap()
        // );
        data_dir.pop();
        data_dir.push("data");
        data_dir.push("anomaly_data");
        data_dir.push("preprocessed");
        let criteria = PartitionCriteria::new(true).with_min_cardinality(cardinality);
        match Self::create_dataset(data_name, &data_dir, distance_metric, is_expensive) {
            Ok(dataset) => {
                let tree = Tree::new(dataset, Some(1))
                    .partition(&criteria)
                    .with_ratios(false);
                Ok(Handle {
                    tree: Some(tree),
                    clam_graph: None,
                    edges: None,
                    current_query: None,
                    force_directed_graph: None,
                })
            }
            Err(_) => Err(FFIError::HandleInitFailed),
        }
    }

    pub fn load_struct(data: &TreeStartupDataFFI) -> Result<Self, FFIError> {
        let data_name = match data.data_name.as_string() {
            Ok(data_name) => data_name,
            Err(e) => {
                debug!("{:?}", e);
                return Err(FFIError::InvalidStringPassed);
            }
        };

        let metric = match utils::distances::from_enum(data.distance_metric) {
            Ok(metric) => metric,
            Err(e) => {
                debug!("{:?}", e);
                return Err(e);
            }
        };
        if let Ok(tree) = Tree::<Vec<f32>, f32, DataSetf32>::load(
            Path::new(&data_name),
            metric,
            data.is_expensive,
        ) {
            let tree = tree.with_ratios(false);
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

    pub fn create_dataset(
        data_name: &str,
        data_dir: &PathBuf,
        distance_metric: DistanceMetric,
        is_expensive: bool,
    ) -> Result<DataSetf32, FFIError> {
        let metric = match utils::distances::from_enum(distance_metric) {
            Ok(metric) => metric,
            Err(e) => {
                debug!("{:?}", e);
                return Err(e);
            }
        };

        match anomaly_readers::read_anomaly_data(data_name, data_dir, false) {
            Ok((first_data, labels)) => {
                // let labels = labels.iter().map(|x| *x == 1).collect::<Vec<bool>>();
                let dataset =
                    VecDataset::new(data_name.to_string(), first_data, metric, is_expensive)
                        .assign_metadata(labels);
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

    pub fn init_clam_graph(
        &'a mut self,
        scoring_function: ScoringFunction,
        min_depth: i32,
        cluster_selector: CBFnNodeVisitor,
    ) -> FFIError {
        if let Some(tree) = &self.tree {
            match enum_to_function(&scoring_function) {
                Ok(scorer) => {
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

    pub fn init_clam_graph_no_visual(
        &'a mut self,
        scoring_function: ScoringFunction,
        min_depth: i32,
    ) -> FFIError {
        if let Some(tree) = &self.tree {
            match enum_to_function(&scoring_function) {
                Ok(scorer) => {
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

    pub unsafe fn force_physics_shutdown(&mut self) -> FFIError {
        if let Some(force_directed_graph) = &self.force_directed_graph {
            force_directed_graph::force_shutdown(&force_directed_graph.1);
            let _ = self.force_directed_graph.take().unwrap().0.join();

            self.force_directed_graph = None;
            debug!("force shutting down physics");
            return FFIError::PhysicsFinished;
        }
        FFIError::PhysicsAlreadyShutdown
    }

    pub unsafe fn init_unity_edges(&mut self, edge_detect_cb: CBFnNodeVisitorMut) -> FFIError {
        if let Some(force_directed_graph) = &self.force_directed_graph {
            force_directed_graph::init_unity_edges(&force_directed_graph.1, edge_detect_cb);
        }
        FFIError::PhysicsAlreadyShutdown
    }

    pub unsafe fn physics_update_async(&mut self, updater: CBFnNodeVisitor) -> FFIError {
        if let Some(force_directed_graph) = &self.force_directed_graph {
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

    pub fn set_graph(&mut self, graph: (JoinHandle<()>, Arc<ForceDirectedGraph>)) {
        self.force_directed_graph = Some(graph);
    }

    pub fn get_num_edges_in_graph(&self) -> i32 {
        if let Some(g) = self.clam_graph() {
            return g.edge_cardinality() as i32;
        }
        return -1;
    }

    pub unsafe fn color_by_dist_to_query(
        &self,
        id_arr: &[String],
        node_visitor: CBFnNodeVisitor,
    ) -> FFIError {
        for id in id_arr {
            match self.get_cluster_from_string(id.clone()) {
                Ok(cluster) => {
                    if let Some(query) = &self.current_query {
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

    pub unsafe fn for_each_dft(
        &self,
        node_visitor: CBFnNodeVisitor,
        start_node: String,
        max_depth: i32,
    ) -> FFIError {
        return if self.tree().is_some() {
            if start_node == "root" {
                if let Some(node) = self.root() {
                    Self::for_each_dft_helper(node, node_visitor, max_depth);
                    FFIError::Ok
                } else {
                    FFIError::HandleInitFailed
                }
            } else {
                match Self::get_cluster_from_string(self, start_node) {
                    Ok(root) => {
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

    pub unsafe fn set_names(
        &self,
        node_visitor: crate::CBFnNameSetter,
        start_node: String,
    ) -> FFIError {
        return if self.tree().is_some() {
            if start_node == "root" {
                if let Some(node) = self.root() {
                    Self::set_names_helper(node, node_visitor);
                    FFIError::Ok
                } else {
                    FFIError::HandleInitFailed
                }
            } else {
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

    fn set_names_helper(root: &Clusterf32, node_visitor: crate::CBFnNameSetter) {
        if root.is_leaf() {
            let baton = ClusterIDsWrapper::from_cluster(root);

            node_visitor(Some(baton.data()));
            return;
        }
        if let Some([left, right]) = root.children() {
            let baton = ClusterIDsWrapper::from_cluster(root);

            node_visitor(Some(baton.data()));
            Self::set_names_helper(left, node_visitor);
            Self::set_names_helper(right, node_visitor);
        }
    }
    fn for_each_dft_helper(root: &Clusterf32, node_visitor: CBFnNodeVisitor, max_depth: i32) {
        if root.is_leaf() || root.depth() as i32 >= max_depth {
            let baton = ClusterDataWrapper::from_cluster(root);
            node_visitor(Some(baton.data()));
            return;
        }
        if let Some([left, right]) = root.children() {
            let baton = ClusterDataWrapper::from_cluster(root);

            node_visitor(Some(baton.data()));

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

    pub fn set_current_query(&mut self, _data: &Vec<f32>) {
        todo!()
        // self.current_query = Some(data.clone());
    }

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

    pub fn get_num_nodes(&self) -> i32 {
        if let Some(tree) = self.tree() {
            tree.cardinality() as i32
        } else {
            0
        }
    }

    pub fn clam_graph(&self) -> Option<&Graph<'a, f32>> {
        self.clam_graph.as_ref()
    }

    pub fn tree_height(&self) -> i32 {
        if let Some(tree) = self.tree() {
            tree.depth() as i32
        } else {
            0
        }
    }

    // why isnt string taken by reference?
    pub unsafe fn get_cluster_from_string(
        &self,
        cluster_id: String,
    ) -> Result<&Clusterf32, FFIError> {
        let mut parts = cluster_id.split('-');

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

    pub unsafe fn get_cluster(
        &self,
        offset: usize,
        cardinality: usize,
    ) -> Result<&Clusterf32, FFIError> {
        if let Some(tree) = self.get_tree() {
            return if let Some(cluster) = tree.get_cluster(offset, cardinality) {
                Ok(cluster)
            } else {
                Err(FFIError::InvalidStringPassed)
            };
        }
        debug!("root not built get cluster");
        Err(FFIError::HandleInitFailed)
    }
    pub fn create_reingold_layout(&self, node_visitor: CBFnNodeVisitor) -> FFIError {
        return if self.tree().is_some() {
            reingold_tilford::run(
                self.root()
                    .unwrap_or_else(|| unreachable!("cakes exists - root should exist")),
                self.get_tree().unwrap().depth() as i32,
                node_visitor,
            )
        } else {
            FFIError::HandleInitFailed
        };
    }

    pub unsafe fn create_reingold_layout_offset_from(
        &self,
        root: &ClusterData,
        _current_depth: i32,
        max_depth: i32,
        node_visitor: CBFnNodeVisitor,
    ) -> FFIError {
        return if self.tree().is_some() {
            if let Ok(clam_root) = self.get_cluster_from_string(root.get_id()) {
                reingold_tilford::run_offset(&root.pos, clam_root, max_depth, node_visitor)
            } else {
                FFIError::NullPointerPassed
            }
        } else {
            FFIError::HandleInitFailed
        };
    }
}
