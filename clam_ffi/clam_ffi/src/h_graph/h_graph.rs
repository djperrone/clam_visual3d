use core::slice;
use std::{
    collections::{HashMap, HashSet},
    f32::consts::PI,
    hash::Hash,
};

use abd_clam::{Cluster, Dataset, Tree};
use float_cmp::approx_eq;
use glam::{vec3, Vec3};
use nalgebra::ComplexField;
use rand::{rngs::ThreadRng, seq::IteratorRandom, Rng};

use crate::utils::{
    error::FFIError,
    types::{Graphf32, Treef32, Vertexf32},
};

use super::{
    h_node::PhysicsNode,
    h_spring::Spring,
    utils::{self, get_children, get_cluster, max_edge_len},
};

pub struct ForceDirectedGraph {
    graph: HashMap<String, PhysicsNode>,
    springs: Vec<Spring>,
    max_iters: usize,
    cur_depth: usize,
    scalar: Option<f32>,
    max_edge_len: Option<f32>,
}

impl ForceDirectedGraph {
    pub fn new(
        tree: &Treef32,
        clam_graph: &Graphf32,
        scalar: f32,
        max_iters: usize,
    ) -> Result<Self, String> {
        Self::build(tree, clam_graph, scalar, max_iters)
    }

    pub fn old_new(
        tree: &Treef32,
        clam_graph: &Graphf32,
        scalar: f32,
        max_iters: usize,
    ) -> Result<Self, String> {
        let mut graph: HashMap<String, PhysicsNode> = HashMap::new();
        let mut rng = rand::thread_rng();

        let area = PI * clam_graph.ordered_clusters().len() as f32;
        // let area = 100.0f32;
        let max_edge_len = max_edge_len(clam_graph.edges());
        for c in clam_graph.ordered_clusters().iter() {
            let x: f32 = rng.gen_range(0.0..=area);
            let y: f32 = rng.gen_range(0.0..=area);
            let z: f32 = rng.gen_range(0.0..=area);
            graph.insert(c.name(), PhysicsNode::new(glam::Vec3::new(x, y, z), c));
        }
        let mut springs = Vec::new();
        for e in clam_graph.edges() {
            springs.push(Spring::new(
                e.distance(),
                e.left().name(),
                e.right().name(),
                0,
                Some(max_edge_len),
                Some(scalar),
            ));
        }

        Ok(Self {
            graph,
            springs,
            max_iters,
            cur_depth: 0,
            scalar: Some(scalar),
            max_edge_len: Some(max_edge_len),
        })
    }

    pub fn run_old_physics(&mut self, tree: &Treef32, clam_graph: &Graphf32) {
        for _ in 0..self.max_iters {
            self.run_single_epoch_old(tree, clam_graph);
        }
    }

    fn build(
        tree: &Treef32,
        clam_graph: &Graphf32,
        scalar: f32,
        max_iters: usize,
    ) -> Result<Self, String> {
        let mut graph: HashMap<String, PhysicsNode> = HashMap::new();
        let mut springs: Vec<Spring> = Vec::new();

        let [left_cluster, right_cluster] = get_children(tree.root())?;
        let normalize_len = max_edge_len(clam_graph.edges());
        let root = PhysicsNode::new(glam::Vec3::new(0., 0., 0.), tree.root());
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

        let (left_node, right_node, spring) =
            Self::split_node(&root, tree.root(), tree, scalar, normalize_len, &mut rng)?;

        graph.insert(left_cluster.name(), left_node);
        graph.insert(right_cluster.name(), right_node);

        springs.push(spring);

        Ok(Self {
            graph,
            springs,
            max_iters,
            cur_depth: 0,
            scalar: Some(scalar),
            max_edge_len: Some(normalize_len),
        })
    }

    pub fn run_physics(&mut self, tree: &Treef32, clam_graph: &Graphf32) -> Result<(), String> {
        let cleanup_interval = 5;
        let relation_threshold = 8;
        for i in 1..clam_graph.max_depth() {
            self.cur_depth = i;
            let nodes: Vec<String> = self.graph.iter().map(|c| c.0.clone()).collect();
            for _ in 0..self.max_iters {
                self.run_single_epoch(&nodes, tree, clam_graph);
            }

            // for (_, node) in self.graph.iter() {
            //     println!("{}", node.get_position().z);
            // }
            // let num_springs_before = self.springs.len();

            let mut next_springs = self.split_nodes(tree, clam_graph)?;
            self.split_springs(&mut next_springs, tree, clam_graph)?;
            self.springs = next_springs;

            // let springs_after_split = self.springs.len();

            // if i == clam_graph.max_depth() - 1 {
            //     self.cleanup(clam_graph, tree, None);
            // } else
            if i % cleanup_interval == 0 {
                self.cleanup(clam_graph, tree, Some(relation_threshold));
            }
        }
        self.cleanup(clam_graph, tree, None);

        // let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

        // let area = PI * clam_graph.ordered_clusters().len() as f32;
        // // let area = 100.0f32;
        // let max_edge_len = max_edge_len(clam_graph.edges());
        // for c in self.graph.iter_mut() {
        //     let x: f32 = rng.gen_range(0.0..=area);
        //     let y: f32 = rng.gen_range(0.0..=area);
        //     let z: f32 = rng.gen_range(0.0..=area);
        //     c.1.set_position(vec3(x, y, z));
        //     // graph.insert(c.name(), PhysicsNode::new(glam::Vec3::new(x, y, z), c));
        // }

        // assert_graphs_equivalent(clam_graph, &self, tree);

        for _ in 0..self.max_iters {
            // self.run_single_epoch();
            self.run_single_epoch_old(tree, clam_graph);
        }

        return Ok(());
    }

    fn run_single_epoch(&mut self, nodes: &Vec<String>, tree: &Treef32, clam_graph: &Graphf32) {
        for spring in &mut self.springs {
            spring.move_nodes(&mut self.graph, None)
        }
        self.accumulate_random_forces2(nodes, clam_graph, tree, self.max_edge_len(), self.scalar());

        for (_, value) in &mut self.graph {
            value.update_position();
        }
    }

    fn run_single_epoch_old(&mut self, tree: &Treef32, clam_graph: &Graphf32) {
        for spring in &mut self.springs {
            spring.move_nodes(&mut self.graph, None)
        }

        self.accumulate_random_forces(clam_graph, tree, self.max_edge_len(), self.scalar());

        for (_, value) in &mut self.graph {
            value.update_position();
        }
    }

    pub fn accumulate_random_forces(
        &mut self,
        clam_graph: &Graphf32,
        tree: &Treef32,
        max_edge_len: f32,
        scalar: f32,
    ) {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        for cluster1 in clam_graph.ordered_clusters() {
            for _ in 0..3 {
                if let Some(cluster2) = clam_graph.ordered_clusters().iter().choose(&mut rng) {
                    if cluster1.name() == cluster2.name() {
                        continue;
                    }
                    let dist = cluster1.distance_to_other(tree.data(), cluster2);

                    let spring = Spring::new(
                        dist,
                        cluster1.name(),
                        cluster2.name(),
                        0,
                        Some(max_edge_len),
                        Some(scalar),
                    );

                    spring.move_nodes(&mut self.graph, None);
                }
            }
        }
    }

    pub fn accumulate_random_forces2(
        &mut self,
        nodes: &Vec<String>,
        clam_graph: &Graphf32,
        tree: &Treef32,
        max_edge_len: f32,
        scalar: f32,
    ) {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        for node1 in nodes {
            for _ in 0..3 {
                if let Some(node2) = nodes.iter().choose(&mut rng) {
                    if node1 == node2 {
                        continue;
                    }

                    let cluster1 = get_cluster(tree, node1).unwrap();
                    let cluster2 = get_cluster(tree, node2).unwrap();
                    let dist = cluster1.distance_to_other(tree.data(), cluster2);

                    let spring = Spring::new(
                        dist,
                        cluster1.name(),
                        cluster2.name(),
                        0,
                        Some(max_edge_len),
                        Some(scalar),
                    );

                    spring.move_nodes(&mut self.graph, None);
                }
            }
        }
    }

    fn split_nodes(
        &mut self,
        tree: &Treef32,
        clam_graph: &Graphf32,
    ) -> Result<Vec<Spring>, String> {
        let mut next_graph: HashMap<String, PhysicsNode> = HashMap::new();
        let mut next_springs: Vec<Spring> = Vec::new();
        let mut rng = rand::thread_rng();

        let area = PI * clam_graph.ordered_clusters().len() as f32;
        // let area = 100.0f32;
        let max_edge_len = max_edge_len(clam_graph.edges());
        let x: f32 = rng.gen_range(0.0..=area);
        for (id, node) in &self.graph {
            let cluster = get_cluster(tree, id)?;
            if clam_graph.ordered_clusters().contains(&cluster) {
                next_graph.insert(cluster.name(), *node);
            } else {
                let (left_node, right_node, spring) = Self::split_node(
                    node,
                    cluster,
                    tree,
                    self.scalar(),
                    self.max_edge_len(),
                    &mut rng,
                )?;
                let (left_id, right_id) = spring.get_node_ids();
                next_graph.insert(left_id.clone(), left_node);
                next_graph.insert(right_id.clone(), right_node);

                next_springs.push(spring);
            }
        }

        self.graph = next_graph;
        return Ok(next_springs);
    }

    fn split_springs(
        &mut self,
        next_springs: &mut Vec<Spring>,
        tree: &Treef32,
        clam_graph: &Graphf32,
    ) -> Result<(), String> {
        for spring in &self.springs {
            let mut clusters: [Vec<&Vertexf32>; 2] = [Vec::new(), Vec::new()];

            // if one node not in graph, find its children in graph
            // repeat for second node
            // for each combination of nodes, calc optimal distance, push spring
            let (left_id, right_id) = spring.get_node_ids();
            if self.graph.contains_key(left_id) && self.graph.contains_key(right_id) {
                next_springs.push(Spring::new(
                    spring.nat_len(),
                    left_id.clone(),
                    right_id.clone(),
                    spring.relation(),
                    None,
                    None,
                ));
                continue;
            }

            if self.graph.contains_key(left_id) {
                let cluster = utils::get_cluster(tree, &left_id)?;
                clusters[0].push(cluster);
            } else {
                let cluster = utils::get_cluster(tree, &left_id)?;
                let [left, right] = get_children(cluster)?;
                clusters[0].push(left);
                clusters[0].push(right);
            }

            if self.graph.contains_key(right_id) {
                let cluster = get_cluster(tree, &right_id)?;
                clusters[1].push(cluster);
            } else {
                let cluster = get_cluster(tree, &right_id)?;
                let [left, right] = get_children(cluster)?;

                clusters[1].push(left);
                clusters[1].push(right);
            }

            for c1 in &clusters[0] {
                for c2 in &clusters[1] {
                    let optimal_distance = utils::cluster_distance_normalized_scaled(
                        c1,
                        c2,
                        tree.data(),
                        self.scalar(),
                        self.max_edge_len(),
                    );

                    next_springs.push(Spring::new(
                        optimal_distance,
                        c1.name(),
                        c2.name(),
                        spring.relation() + 1,
                        None,
                        None,
                    ));
                }
            }
        }
        Ok(())
    }

    fn cleanup_helper(
        spring: &Spring,
        tree: &Treef32,
        relation_threshold: Option<u32>,
    ) -> Result<bool, String> {
        if let Some(relation_threshold) = relation_threshold {
            if spring.relation() > relation_threshold {
                let ids = spring.get_node_ids();
                let c1 = get_cluster(tree, ids.0)?;

                let c2 = get_cluster(tree, ids.1)?;
                let distance = c1.distance_to_other(tree.data(), c2);
                if distance > c1.radius() + c2.radius() {
                    return Ok(false);
                }
            }
        } else {
            let ids = spring.get_node_ids();
            let c1 = get_cluster(tree, ids.0)?;

            let c2 = get_cluster(tree, ids.1)?;
            let distance = c1.distance_to_other(tree.data(), c2);
            if distance > c1.radius() + c2.radius() {
                return Ok(false);
            }
        }

        return Ok(true);
    }

    fn cleanup(&mut self, clam_graph: &Graphf32, tree: &Treef32, relation: Option<u32>) {
        self.springs
            .retain(|spring| Self::cleanup_helper(spring, tree, relation).unwrap());
    }

    pub fn split_node<'a>(
        node: &'a PhysicsNode,
        cluster: &'a Vertexf32,
        tree: &'a Treef32,
        scalar: f32,
        max_edge_len: f32,
        rng: &mut ThreadRng,
    ) -> Result<(PhysicsNode, PhysicsNode, Spring), String> {
        let [left, right] = get_children(cluster)?;
        if let Some((dist_between_children, dist_to_left_child, dist_to_right_child)) =
            utils::calc_triangle_distances(cluster, left, right, tree, scalar, max_edge_len)
        {
            let root_pos = node.get_position();

            // let left_pos = vec3(root_pos.x - dist_to_left_child, root_pos.y, root_pos.z);

            let left_x: f32 =
                rng.gen_range(root_pos.x - dist_to_left_child..=root_pos.x + dist_to_left_child);
            let left_y: f32 =
                rng.gen_range(root_pos.y - dist_to_left_child..=root_pos.y + dist_to_left_child);
            let left_z: f32 =
                rng.gen_range(root_pos.z - dist_to_left_child..=root_pos.z + dist_to_left_child);

            let left_pos = vec3(left_x, left_y, left_z);

            let right_x: f32 =
                rng.gen_range(root_pos.x - dist_to_right_child..=root_pos.x + dist_to_right_child);
            let right_y: f32 =
                rng.gen_range(root_pos.y - dist_to_right_child..=root_pos.y + dist_to_right_child);
            let right_z: f32 =
                rng.gen_range(root_pos.z - dist_to_right_child..=root_pos.z + dist_to_right_child);

            let right_pos = vec3(right_x, right_y, right_z);

            // let right_pos = utils::calc_circle_intersection(
            //     &root_pos,
            //     dist_to_right_child,
            //     &left_pos,
            //     dist_between_children,
            //     dist_to_left_child,
            // )?;

            return Ok((
                PhysicsNode::new(left_pos, left),
                PhysicsNode::new(right_pos, right),
                Spring::new(
                    dist_between_children,
                    left.name(),
                    right.name(),
                    0,
                    None,
                    None,
                ),
            ));
        } else {
            return Err("Could not calculate triangle distances".to_string());
        }
    }

    pub fn scalar(&self) -> f32 {
        self.scalar.unwrap_or(1.0)
    }

    pub fn max_edge_len(&self) -> f32 {
        self.max_edge_len.unwrap_or(1.0)
    }

    pub fn get_cluster_position(&self, id: &str) -> Result<Vec3, String> {
        if let Some(c) = self.graph.get(id) {
            return Ok(c.get_position());
        } else {
            return Err("Cluster not found".to_string());
        }
    }

    pub fn contains(&self, id: &str) -> bool {
        self.graph.contains_key(id)
    }
}

fn assert_graphs_equivalent(clam_graph: &Graphf32, fdg: &ForceDirectedGraph, tree: &Treef32) {
    assert_eq!(clam_graph.ordered_clusters().len(), fdg.graph.len());
    assert_eq!(clam_graph.edges().len(), fdg.springs.len());

    for c in clam_graph.ordered_clusters() {
        assert!(fdg.contains(&c.name()));
    }

    for c in fdg.graph.iter() {
        let cluster = get_cluster(tree, c.0).unwrap();
        assert!(clam_graph.ordered_clusters().contains(&cluster));
    }

    for edge in clam_graph.edges() {
        let (left, right) = (edge.left(), edge.right());
        assert!(fdg_contains_spring(fdg, &left.name(), &right.name()));
    }
}

fn fdg_contains_spring(fdg: &ForceDirectedGraph, left: &str, right: &str) -> bool {
    for spring in &fdg.springs {
        let (l, r) = spring.get_node_ids();

        if (l == left && r == right) || (l == right && r == left) {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use std::fs::{self, ReadDir};
    use std::path::{Path, PathBuf};

    use crate::h_graph;
    use crate::handle::handle::Handle;
    use crate::utils::distances::DistanceMetric;
    use crate::utils::scoring_functions::{enum_to_function, ScoringFunction};

    use super::*;
    use abd_clam::graph::{Graph, MetaMLScorer};
    use abd_clam::{graph, PartitionCriteria};
    use float_cmp::{approx_eq, assert_approx_eq};
    use float_cmp::{ApproxEq, F64Margin};
    use glam::{vec3, Vec3};

    #[test]
    fn physics() {
        let (
            dir,
            min_cardinality,
            min_depth,
            distance_metric,
            scalar,
            max_iters,
            src_folder,
            out_folder_root,
            target,
            // ) = test_params(None);
        ) = test_params(Some("wine".to_string()));

        // let outfolder = "angle_distortion";
        let mut out_folder = PathBuf::new();
        out_folder.push(out_folder_root);
        out_folder.push("fnn");

        let targets = vec![
            // "arrhythmia".to_string(),
            "satellite".to_string(),
            // "wine".to_string(),
        ];

        for target in targets {
            run_single_target(
                &target,
                &src_folder,
                out_folder.to_str().unwrap(),
                distance_metric,
                min_cardinality,
                max_iters,
                scalar,
            )
        }
    }

    fn run_test_on_file(
        filename: &str,
        src_folder: &PathBuf,
        out_folder: &str,
        distance_metric: DistanceMetric,
        min_cardinality: usize,
        max_iters: i32,
        scalar: f32,
    ) {
        match Handle::create_dataset(filename, &src_folder, distance_metric, false) {
            Ok(data) => {
                println!("created dataset");
                let criteria = PartitionCriteria::new(true).with_min_cardinality(min_cardinality);

                let tree = Tree::new(data, Some(1)).partition(&criteria, None);
                for depth in 4..10 as usize {
                    if let Ok(graph) = Graph::from_tree(
                        &tree,
                        &enum_to_function(&ScoringFunction::LrEuclideanCc).unwrap(),
                        depth,
                    ) {
                        if let Ok(fdg) = run_physics_sim(&tree, &graph, scalar, max_iters) {
                            println!("asserting equal");
                            assert_graphs_equivalent(&graph, &fdg, &tree);
                        } else {
                            panic!("collecting data for this graph failed");
                        }
                    }
                }
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    fn assert_graphs_equivalent(clam_graph: &Graphf32, fdg: &ForceDirectedGraph, tree: &Treef32) {
        assert_eq!(clam_graph.ordered_clusters().len(), fdg.graph.len());
        assert_eq!(clam_graph.edges().len(), fdg.springs.len());

        for c in clam_graph.ordered_clusters() {
            assert!(fdg.contains(&c.name()));
        }

        for c in fdg.graph.iter() {
            let cluster = get_cluster(tree, c.0).unwrap();
            assert!(clam_graph.ordered_clusters().contains(&cluster));
        }

        for edge in clam_graph.edges() {
            let (left, right) = (edge.left(), edge.right());
            assert!(fdg_contains_spring(fdg, &left.name(), &right.name()));
        }
    }

    fn fdg_contains_spring(fdg: &ForceDirectedGraph, left: &str, right: &str) -> bool {
        for spring in &fdg.springs {
            let (l, r) = spring.get_node_ids();

            if (l == left && r == right) || (l == right && r == left) {
                return true;
            }
        }
        return false;
    }

    fn run_physics_sim(
        tree: &Treef32,
        graph: &Graphf32,
        scalar: f32,
        max_iters: i32,
    ) -> Result<ForceDirectedGraph, String> {
        let mut fdg =
            h_graph::h_graph::ForceDirectedGraph::new(tree, &graph, scalar, max_iters as usize)?;

        fdg.run_physics(tree, graph)?;
        // let mut fdg = build_force_directed_graph(&tree, &graph, scalar, max_iters);
        println!("created fdg");

        Ok(fdg)

        // Ok((precision_results, recall_results, f1_score_results))
    }

    fn run_single_target(
        filename: &str,
        src_folder: &PathBuf,
        out_folder: &str,
        distance_metric: DistanceMetric,
        min_cardinality: usize,
        max_iters: i32,
        scalar: f32,
    ) {
        run_test_on_file(
            filename,
            &src_folder,
            out_folder,
            distance_metric,
            min_cardinality,
            max_iters,
            scalar,
        );
    }

    #[test]
    fn node_split() {
        let data = utils::gen_dataset_from(
            vec![
                vec![10.],
                vec![1.],
                vec![-5.],
                vec![8.],
                vec![3.],
                vec![2.],
                vec![0.5],
                vec![0.],
            ],
            utils::euclidean::<f32, f32>,
            vec![1, 1, 0, 0, 1, 0, 1, 0],
        );
        let criteria = PartitionCriteria::default();

        let tree = Treef32::new(data, Some(42)).partition(&criteria, Some(42));
        let scalar = 100.0;
        let graph = Graph::from_tree(
            &tree,
            &enum_to_function(&ScoringFunction::LrEuclideanCc).unwrap(),
            2,
        )
        .unwrap();
        let normalize_len = max_edge_len(graph.edges());

        let root_node = PhysicsNode::new(vec3(-1000., -100., 150.), tree.root());
        let [left, right] = tree.root().children().unwrap();
        let (dist_between_children, dist_to_left_child, dist_to_right_child) =
            utils::calc_triangle_distances(tree.root(), left, right, &tree, scalar, normalize_len)
                .unwrap();
        let root_pos = root_node.get_position();
        // let left_pos = vec3(root_pos.x - dist_to_left_child, root_pos.y, root_pos.z);

        // println!(
        //     "distances: {}, {}, {}",
        //     dist_between_children, dist_to_left_child, dist_to_right_child
        // );
        // let right_pos = ForceDirectedGraph::calc_circle_intersection(
        //     &root_pos,
        //     dist_to_right_child,
        //     &left_pos,
        //     dist_between_children,
        //     dist_to_left_child,
        // )
        // .unwrap();
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

        let (left_child, right_child, _) = ForceDirectedGraph::split_node(
            &root_node,
            tree.root(),
            &tree,
            scalar,
            normalize_len,
            &mut rng,
        )
        .unwrap();
        assert!(approx_eq!(
            f32,
            root_pos.distance(left_child.get_position()),
            dist_to_left_child,
            epsilon = 0.000003
        ));

        assert!(approx_eq!(
            f32,
            root_pos.distance(right_child.get_position()),
            dist_to_right_child,
            epsilon = 0.000003
        ));

        println!(
            "{}, {}",
            left_child
                .get_position()
                .distance(right_child.get_position()),
            dist_between_children
        );

        assert!(approx_eq!(
            f32,
            left_child
                .get_position()
                .distance(right_child.get_position()),
            dist_between_children,
            epsilon = 0.000003
        ));

        assert_valid_triangle(
            left_child.get_position(),
            right_child.get_position(),
            root_node.get_position(),
        );
    }

    fn assert_valid_triangle_area(p1: Vec3, p2: Vec3, p3: Vec3) -> bool {
        // Calculate the area of the triangle using the determinant method
        let area = 0.5 * ((p2.x - p1.x) * (p3.y - p1.y) - (p3.x - p1.x) * (p2.y - p1.y)).abs();
        !float_cmp::approx_eq!(f32, area, 0.0, epsilon = 0.000003)
    }

    fn assert_triangle_inequality_theoreum(p1: Vec3, p2: Vec3, p3: Vec3) {
        let a = p1.distance(p2);
        let b = p2.distance(p3);
        let c = p3.distance(p1);

        println!("acb {} >{}", a + c, b);
        println!("bca {} >{}", c + b, a);
        println!("abc {} >{}", a + b, c);

        // Check for the triangle inequality theorem
        let valid = a + b > c || a + c > b || b + c > a;

        assert!(valid);

        assert_valid_triangle_area(p1, p2, p3);
    }

    fn assert_valid_triangle(p1: Vec3, p2: Vec3, p3: Vec3) {
        assert_valid_triangle_area(p1, p2, p3);
        assert_triangle_inequality_theoreum(p1, p2, p3);
    }

    fn test_params(
        single_target: Option<String>,
    ) -> (
        ReadDir,
        usize,
        usize,
        DistanceMetric,
        f32,
        i32,
        PathBuf,
        String,
        Option<String>,
    ) {
        let dir_path = Path::new("../../data/anomaly_data/preprocessed");

        // Open the directory
        let data_folder = fs::read_dir(dir_path).unwrap();
        let min_cardinality = 1;
        let min_depth = 11;
        let distance_metric = DistanceMetric::Euclidean;
        let scalar = 100.0;
        let max_iters = 1200;
        let data_folder_name = PathBuf::from(dir_path);
        // let k: usize = ;

        (
            data_folder,
            min_cardinality,
            min_depth,
            distance_metric,
            scalar,
            max_iters,
            data_folder_name,
            String::from("accuracy_results"),
            single_target,
            // single_target,
        )
    }
}
