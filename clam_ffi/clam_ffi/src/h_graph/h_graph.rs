use std::{
    collections::{HashMap, HashSet},
    f32::consts::PI,
    hash::Hash,
};

use abd_clam::{Cluster, Dataset, Tree};
use float_cmp::approx_eq;
use glam::{vec3, Vec3};
use nalgebra::ComplexField;
use rand::Rng;

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

    fn build(
        tree: &Treef32,
        clam_graph: &Graphf32,
        scalar: f32,
        max_iters: usize,
    ) -> Result<Self, String> {
        let mut graph: HashMap<String, PhysicsNode> = HashMap::new();
        let mut springs: Vec<Spring> = Vec::new();
        let normalize_len = max_edge_len(clam_graph.edges());

        let [left_cluster, right_cluster] = get_children(tree.root())?;

        let normalize_len = max_edge_len(clam_graph.edges());
        let root = PhysicsNode::new(glam::Vec3::new(0., 0., 0.), tree.root());
        let (left_node, right_node, spring) =
            Self::split_node(&root, tree.root(), tree, scalar, normalize_len)?;

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

    pub fn update(&mut self, tree: &Treef32, clam_graph: &Graphf32) -> Result<(), String> {
        let cleanup_interval = 3;
        for i in 1..clam_graph.max_depth() + 1 {
            self.cur_depth = i;

            for _ in 0..self.max_iters {
                self.run_physics();
            }
            self.split_nodes(tree, clam_graph)?;

            if i % cleanup_interval == 0 {
                self.cleanup(clam_graph, tree, Some(4));
            }
        }

        if clam_graph.max_depth() + 1 % cleanup_interval != 0 {
            self.cleanup(clam_graph, tree, None);
        }

        return Ok(());
    }

    pub fn run_physics(&mut self) {
        for spring in &mut self.springs {
            spring.move_nodes(&mut self.graph, None)
        }

        for (_, value) in &mut self.graph {
            value.update_position();
        }
    }

    fn split_nodes(&mut self, tree: &Treef32, clam_graph: &Graphf32) -> Result<(), String> {
        let mut next_graph: HashMap<String, PhysicsNode> = HashMap::new();
        let mut next_springs: Vec<Spring> = Vec::new();
        for (id, node) in &self.graph {
            let cluster = get_cluster(tree, id)?;
            if clam_graph.ordered_clusters().contains(&cluster) {
                next_graph.insert(cluster.name(), *node);
            } else {
                let (left_node, right_node, spring) =
                    Self::split_node(node, cluster, tree, self.scalar(), self.max_edge_len())?;
                let (left_id, right_id) = spring.get_node_ids();
                next_graph.insert(left_id.clone(), left_node);
                next_graph.insert(right_id.clone(), right_node);

                next_springs.push(spring);
            }
        }

        self.graph = next_graph;
        return Ok(());
    }

    fn split_springs(&mut self, tree: &Treef32, clam_graph: &Graphf32) -> Result<(), String> {
        let mut next_springs: Vec<Spring> = Vec::new();

        for spring in &self.springs {
            let mut clusters: Vec<Vec<&Vertexf32>> = Vec::with_capacity(2);

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
        if spring.relation() >= relation_threshold.unwrap_or(u32::MIN) {
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
    ) -> Result<(PhysicsNode, PhysicsNode, Spring), String> {
        let [left, right] = get_children(cluster)?;
        if let Some((dist_between_children, dist_to_left_child, dist_to_right_child)) =
            utils::calc_triangle_distances(cluster, left, right, tree, scalar, max_edge_len)
        {
            let root_pos = node.get_position();
            let left_pos = vec3(root_pos.x - dist_to_left_child, root_pos.y, root_pos.z);

            let right_pos = utils::calc_circle_intersection(
                &root_pos,
                dist_to_right_child,
                &left_pos,
                dist_between_children,
                dist_to_left_child,
            )?;

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
}

#[cfg(test)]
mod tests {
    use crate::h_graph;
    use crate::utils::scoring_functions::{enum_to_function, ScoringFunction};

    use super::*;
    use abd_clam::graph::{Graph, MetaMLScorer};
    use abd_clam::{graph, PartitionCriteria};
    use float_cmp::{approx_eq, assert_approx_eq};
    use float_cmp::{ApproxEq, F64Margin};
    use glam::{vec3, Vec3};

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
        let (left_child, right_child, _) =
            ForceDirectedGraph::split_node(&root_node, tree.root(), &tree, scalar, normalize_len)
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
}
