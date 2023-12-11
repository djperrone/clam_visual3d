#![allow(dead_code)]
#![allow(unused_variables)]

use super::string_ffi::StringFFI;
use crate::tree_layout::reingold_impl;
use crate::utils::types::Clusterf32;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ClusterData {
    pub depth: i32,
    pub offset: i32,
    pub cardinality: i32,
    pub arg_center: i32,
    pub arg_radial: i32,
    pub radius: f32,
    pub lfd: f32,

    pub vertex_degree: i32,
    pub dist_to_query: f32,

    pub pos: glam::Vec3,
    pub color: glam::Vec3,

    pub id: StringFFI,
    pub message: StringFFI,
}

impl ClusterData {
    pub fn default() -> Self {
        ClusterData {
            id: StringFFI::new("".to_string()),
            color: glam::Vec3::new(0., 0., 0.),
            pos: glam::Vec3::new(0., 0., 0.),
            cardinality: -1,
            depth: -1,
            offset: -1,
            radius: -1.0,
            lfd: -1.0,
            arg_center: -1,
            arg_radial: -1,
            vertex_degree: -1,
            dist_to_query: -1f32,
            message: StringFFI::new("".repeat(50)),
        }
    }
    pub fn from_physics(id: String, position: glam::Vec3) -> Self {
        ClusterData {
            id: StringFFI::new(id),
            color: glam::Vec3::new(0., 0., 0.),
            pos: position,
            cardinality: -1,
            offset: -1,
            depth: -1,
            radius: -1.0,
            lfd: -1.0,
            arg_center: -1,
            arg_radial: -1,
            vertex_degree: -1,

            dist_to_query: -1f32,
            message: StringFFI::new("".repeat(50)),
        }
    }

    pub fn set_message(&mut self, msg: String) {
        self.message.free_data();
        self.message = StringFFI::new(msg);
    }

    pub fn set_id(&mut self, msg: String) {
        self.id.free_data();
        self.id = StringFFI::new(msg);
    }

    pub unsafe fn get_id(&self) -> String {
        self.id.as_string().unwrap()
    }

    pub unsafe fn get_ffi_id(&self) -> &StringFFI {
        &self.id
    }
    pub fn set_position(&mut self, pos: glam::Vec3) {
        self.pos = pos;
    }

    pub fn set_color(&mut self, color: glam::Vec3) {
        self.color = color;
    }

    pub fn from_clam(node: &Clusterf32) -> Self {
        let (left_id, right_id) = {
            if let Some([left, right]) = node.children() {
                (left.name(), right.name())
            } else {
                ("None".to_string(), "None".to_string())
            }
        };

        ClusterData {
            pos: glam::Vec3::new(0., 0., 0.),
            color: glam::Vec3::new(0., 0., 0.),
            id: (StringFFI::new(node.name())),
            cardinality: (node.cardinality() as i32),
            offset: (node.offset() as i32),
            depth: (node.depth() as i32),
            radius: node.radius(),
            lfd: node.lfd() as f32,
            arg_center: (node.arg_center() as i32),
            arg_radial: (node.arg_radial() as i32),
            vertex_degree: -1,

            dist_to_query: -1f32,
            message: StringFFI::new("".repeat(50)),
        }
    }

    pub fn from_reingold_node(other: &reingold_impl::Node) -> Self {
        let (left, right) = other.get_child_names();
        ClusterData {
            pos: glam::Vec3::new(other.get_x(), other.get_y(), 0.),
            color: glam::Vec3::new(0., 0., 0.),
            id: StringFFI::new(other.get_name()),
            cardinality: -1,
            offset: -1,
            depth: other.depth(),
            radius: -1.0,
            lfd: -1.0,
            arg_center: -1,
            arg_radial: -1,
            vertex_degree: -1,

            dist_to_query: -1f32,
            message: StringFFI::new("".repeat(50)),
        }
    }
    pub fn free_ids(&mut self) {
        self.id.free_data();
        self.message.free_data();
    }
}
