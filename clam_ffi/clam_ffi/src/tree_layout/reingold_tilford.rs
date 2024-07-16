use glam::Vec3;

use crate::{
    // ffi_impl::cluster_data_wrapper::ClusterDataWrapper,
    ffi_impl::cluster_data::ClusterData, utils::{error::FFIError, types::Vertexf32}
};

use super::reingold_impl;

pub fn run(
    clam_root: &Vertexf32,
    max_depth: i32,
    node_visitor: crate::CBFnNodeVisitor,
) -> FFIError {
    let layout_root = reingold_impl::Node::create_layout(clam_root, max_depth);
    update_unity_positions(layout_root, node_visitor)
}

pub fn run_offset(
    start_pos: &Vec3,
    clam_root: &Vertexf32,
    max_depth: i32,
    node_visitor: crate::CBFnNodeVisitor,
) -> FFIError {
    let layout_root = reingold_impl::Node::create_layout(clam_root, max_depth);
    update_unity_positions_offset(layout_root, start_pos, node_visitor, max_depth)
}

fn update_unity_positions_offset(
    root: reingold_impl::Link,
    start_pos: &Vec3,
    node_visitor: crate::CBFnNodeVisitor,
    max_depth: i32,
) -> FFIError {
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
    FFIError::NullPointerPassed
}

fn update_helper_offset(
    root: reingold_impl::Link,
    offset: &glam::Vec3,
    node_visitor: crate::CBFnNodeVisitor,
    max_depth: i32,
) {
    if max_depth == -2 {
        return;
    }
    if let Some(node) = root {
        let mut data = ClusterData::from_reingold_node(&node.as_ref().borrow());
        data.pos.x += offset.x;
        data.pos.y -= offset.y;
        data.pos.z += offset.z;
        node_visitor(Some(&data));

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
    if root.clone().is_some() {
        update_helper(root.clone(), node_visitor);

        return FFIError::Ok;
    }
    FFIError::NullPointerPassed
}

fn update_helper(root: reingold_impl::Link, node_visitor: crate::CBFnNodeVisitor) {
    if let Some(node) = root {
        let data = ClusterData::from_reingold_node(&node.as_ref().borrow());

        node_visitor(Some(&data));
        update_helper(node.as_ref().borrow().get_left_child(), node_visitor);
        update_helper(node.as_ref().borrow().get_right_child(), node_visitor);
    }
}
