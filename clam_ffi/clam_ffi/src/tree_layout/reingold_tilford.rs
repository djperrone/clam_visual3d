// use

use glam::Vec3;
use nalgebra::max;

use crate::{
    ffi_impl::cluster_data_wrapper::ClusterDataWrapper,
    utils::{error::FFIError, types::Clusterf32},
    // CBFnNodeVisitor,
};

use super::reingold_impl;

// pub fn create_layout() -> () {}

pub fn run(
    clam_root: &Clusterf32,
    labels: &Option<Vec<bool>>,
    max_depth: i32,
    node_visitor: crate::CBFnNodeVisitor,
) -> FFIError {
    // if let Some(labels) = &self.labels {
    let layout_root = reingold_impl::Node::create_layout(clam_root, max_depth);
    let result = update_unity_positions(layout_root, node_visitor);

    return result;
}

pub fn run_offset(
    start_pos: &Vec3,
    clam_root: &Clusterf32,
    // labels: &Option<Vec<u8>>,
    // current_depth: i32,
    max_depth: i32,
    node_visitor: crate::CBFnNodeVisitor,
) -> FFIError {
    // if let Some(labels) = &self.labels {
    let layout_root = reingold_impl::Node::create_layout(clam_root, max_depth);
    let result = update_unity_positions_offset(layout_root, start_pos, node_visitor, max_depth);

    return result;
}

fn update_unity_positions_offset(
    root: reingold_impl::Link,
    start_pos: &Vec3,
    node_visitor: crate::CBFnNodeVisitor,
    max_depth: i32,
) -> FFIError {
    // if max_depth == -1{
    //     return FFIError::Ok;
    // }
    if let Some(node) = root.clone() {
        let (x, y, z) = (
            node.as_ref().borrow().get_x(),
            node.as_ref().borrow().get_y(),
            0.0,
        );
        let offset = glam::Vec3::new(start_pos.x - x, start_pos.y - y, start_pos.z - z);

        update_helper_offset(root.clone(), &offset, node_visitor, max_depth - 1);

        return FFIError::Ok;
    }
    return FFIError::NullPointerPassed;
}

fn update_helper_offset(
    root: reingold_impl::Link,
    offset: &glam::Vec3,
    node_visitor: crate::CBFnNodeVisitor,
    max_depth: i32,
) -> () {
    if max_depth == -2 {
        return;
    }
    if let Some(node) = root {
        let mut baton = ClusterDataWrapper::from_reingold_node(&node.as_ref().borrow());
        baton.data_mut().pos.x += offset.x;
        baton.data_mut().pos.y -= offset.y;
        baton.data_mut().pos.z += offset.z;
        node_visitor(Some(baton.data()));
        // baton.free_ids();

        update_helper_offset(
            node.as_ref().borrow().get_left_child(),
            offset,
            node_visitor,
            max_depth - 1,
        );
        update_helper_offset(
            node.as_ref().borrow().get_right_child(),
            offset,
            node_visitor,
            max_depth - 1,
        );
    }
}

fn update_unity_positions(
    root: reingold_impl::Link,
    node_visitor: crate::CBFnNodeVisitor,
) -> FFIError {
    if let Some(_) = root.clone() {
        update_helper(root.clone(), node_visitor);

        return FFIError::Ok;
    }
    return FFIError::NullPointerPassed;
}

fn update_helper(root: reingold_impl::Link, node_visitor: crate::CBFnNodeVisitor) -> () {
    if let Some(node) = root {
        let baton = ClusterDataWrapper::from_reingold_node(&node.as_ref().borrow());

        node_visitor(Some(baton.data()));
        // baton.free_ids();

        update_helper(node.as_ref().borrow().get_left_child(), node_visitor);
        update_helper(node.as_ref().borrow().get_right_child(), node_visitor);
    }
}
