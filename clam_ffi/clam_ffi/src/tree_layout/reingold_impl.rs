#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

use abd_clam::{graph::Vertex, Cluster};
use std::{cell::RefCell, rc::Rc};

use crate::utils::types::Vertexf32;

extern crate nalgebra as na;
type Vec3 = na::Vector3<f32>;
const MIN_SEP: f32 = 1f32;

pub type Link = Option<Rc<RefCell<Node>>>;
type ExtremeLink = Rc<RefCell<Extreme>>;

struct Extreme {
    addr: Link,
    offset: f32,
    level: f32,
}

impl Extreme {
    pub fn default() -> Self {
        Extreme {
            addr: None,
            offset: -1f32,
            level: -1f32,
        }
    }

    pub fn default_link() -> ExtremeLink {
        Rc::new(RefCell::new(Extreme::default()))
    }

    pub fn copy(&mut self, other: ExtremeLink) {
        self.addr = other.as_ref().borrow().addr.clone();
        self.offset = other.as_ref().borrow().offset;
        self.level = other.as_ref().borrow().level;
    }
}

pub struct Node {
    x: f32,
    y: f32,
    offset: f32,
    thread: bool,
    left_child: Link,
    right_child: Link,
    name: String,
}

impl Node {
    pub fn new(depth: f32, name: String) -> Self {
        Node {
            x: 0f32,
            y: depth,
            offset: 0f32,
            thread: false,
            left_child: None,
            right_child: None,
            name,
        }
    }

    pub fn new_link(depth: f32, name: String) -> Link {
        Some(Rc::new(RefCell::new(Node::new(depth, name))))
    }

    pub fn create_layout(abd_clam_root: &Vertexf32, max_depth: i32) -> Link {
        let draw_root = Node::new_link(0f32, abd_clam_root.name());

        Self::init_helper(draw_root.clone(), abd_clam_root, 0f32, max_depth);

        Self::setup(
            draw_root.clone(),
            0f32,
            Extreme::default_link(),
            Extreme::default_link(),
            max_depth,
        );
        Self::petrify(draw_root.clone(), 0f32, max_depth);

        draw_root
    }

    fn init_helper(draw_root: Link, abd_clam_root: &Vertexf32, depth: f32, max_depth: i32) {
        if abd_clam_root.is_leaf() || depth as i32 == max_depth {
            return;
        }

        if let Some([left, right]) = abd_clam_root.children() {
            if let Some(node) = draw_root.clone() {
                node.borrow_mut().left_child = Node::new_link(depth, left.name());
                node.borrow_mut().right_child = Node::new_link(depth, right.name());
                Self::init_helper(
                    node.as_ref().borrow().get_left_child(),
                    left,
                    depth + 1.,
                    max_depth,
                );
                Self::init_helper(
                    node.as_ref().borrow().get_right_child(),
                    right,
                    depth + 1.,
                    max_depth,
                );
            }
        }
    }

    fn setup(t: Link, level: f32, right_most: ExtremeLink, left_most: ExtremeLink, depth: i32) {
        if depth == -2 {
            return;
        }
        let (lr, ll, rr, rl) = (
            Extreme::default_link(),
            Extreme::default_link(),
            Extreme::default_link(),
            Extreme::default_link(),
        );

        if t.is_none() {
            left_most.borrow_mut().level = -1.;
            right_most.borrow_mut().level = -1.;
        } else if let Some(node) = t.clone() {
            node.borrow_mut().y = level;

            let (mut left_child, mut right_child) = node.as_ref().borrow().get_children();
            Self::setup(
                left_child.clone(),
                level + 1.,
                lr.clone(),
                ll.clone(),
                depth - 1,
            );
            Self::setup(
                right_child.clone(),
                level + 1.,
                rr.clone(),
                rl.clone(),
                depth - 1,
            );

            if left_child.is_none() && right_child.is_none() {
                node.borrow_mut().offset = 0.;

                right_most.borrow_mut().addr = Some(node.clone());
                right_most.borrow_mut().level = level;
                right_most.borrow_mut().offset = 0.;
                left_most.borrow_mut().addr = Some(node.clone());
                left_most.borrow_mut().level = level;
                left_most.borrow_mut().offset = 0.;
            } else {
                let (mut current_sep, mut root_sep, mut loffsum, mut roffsum) =
                    (MIN_SEP, MIN_SEP, 0f32, 0f32);

                while left_child.is_some() && right_child.is_some() {
                    if current_sep < MIN_SEP {
                        root_sep += MIN_SEP - current_sep;
                        current_sep = MIN_SEP;
                    }

                    if let Some(left) = left_child.clone() {
                        if let Some(inner_left) = left.as_ref().borrow().get_right_child() {
                            loffsum += left.as_ref().borrow().offset;
                            current_sep -= left.as_ref().borrow().offset;
                            left_child = Some(inner_left);
                        } else {
                            loffsum -= left.as_ref().borrow().offset;
                            current_sep += left.as_ref().borrow().offset;
                            let inner_left = left.as_ref().borrow().get_left_child();
                            left_child = inner_left.clone();
                        }
                    } else {
                        panic!();
                    }

                    if let Some(right) = right_child.clone() {
                        if let Some(inner_right) = right.as_ref().borrow().get_left_child() {
                            roffsum -= right.as_ref().borrow().offset;
                            current_sep -= right.as_ref().borrow().offset;
                            right_child = Some(inner_right);
                        } else {
                            roffsum += right.as_ref().borrow().offset;
                            current_sep += right.as_ref().borrow().offset;
                            let inner_right = right.as_ref().borrow().get_right_child();
                            right_child = inner_right.clone();
                        }
                    } else {
                        panic!();
                    }
                } //end while

                node.borrow_mut().offset = (root_sep + 1.) / 2.;
                loffsum -= node.as_ref().borrow().offset;
                roffsum += node.as_ref().borrow().offset;

                if rl.as_ref().borrow().level > ll.as_ref().borrow().level
                    || node.as_ref().borrow().get_left_child().is_none()
                {
                    left_most.borrow_mut().copy(rl.clone());
                    left_most.borrow_mut().offset += node.as_ref().borrow().offset;
                } else {
                    left_most.borrow_mut().copy(ll.clone());
                    left_most.borrow_mut().offset -= node.as_ref().borrow().offset;
                }

                if lr.as_ref().borrow().level > rr.as_ref().borrow().level
                    || node.as_ref().borrow().get_right_child().is_none()
                {
                    right_most.borrow_mut().copy(lr.clone());
                    right_most.borrow_mut().offset -= node.as_ref().borrow().offset;
                } else {
                    right_most.borrow_mut().copy(rr.clone());
                    right_most.borrow_mut().offset += node.as_ref().borrow().offset;
                }

                if let Some(left) = left_child.clone() {
                    let node_left_name = {
                        if let Some(node_left) = node.as_ref().borrow().get_left_child() {
                            node_left.as_ref().borrow().name.clone()
                        } else {
                            String::new()
                        }
                    };

                    if left.as_ref().borrow().name != node_left_name {
                        if let Some(addr) = rr.as_ref().borrow().addr.clone() {
                            let mut addr = addr.borrow_mut();
                            addr.thread = true;
                            let offset = (rr.as_ref().borrow().offset
                                + node.as_ref().borrow().offset)
                                - loffsum;
                            addr.offset = offset.abs();

                            if loffsum - node.as_ref().borrow().offset
                                <= rr.as_ref().borrow().offset
                            {
                                addr.left_child = left_child.clone();
                            } else {
                                addr.right_child = left_child.clone();
                            }
                        }
                    }
                } else if let Some(right) = right_child.clone() {
                    let node_right_name = {
                        if let Some(node_right) = node.as_ref().borrow().get_right_child() {
                            node_right.as_ref().borrow().name.clone()
                        } else {
                            String::new()
                        }
                    };
                    if right.as_ref().borrow().name != node_right_name {
                        if let Some(addr) = ll.as_ref().borrow().addr.clone() {
                            let mut addr = addr.borrow_mut();
                            addr.thread = true;
                            let offset = (ll.as_ref().borrow().offset
                                - node.as_ref().borrow().offset)
                                - roffsum;
                            addr.offset = offset.abs();

                            if roffsum + node.as_ref().borrow().offset
                                >= ll.as_ref().borrow().offset
                            {
                                addr.right_child = right_child.clone();
                            } else {
                                addr.left_child = right_child.clone();
                            }
                        }
                    }
                }
            }
        }
    }

    fn petrify(t: Link, x: f32, max_depth: i32) {
        if max_depth == -2 {
            return;
        }

        if let Some(node) = t {
            node.borrow_mut().x = x;
            if node.as_ref().borrow().thread {
                node.borrow_mut().left_child = None;
                node.borrow_mut().right_child = None;
                node.borrow_mut().thread = false;
            }
            Self::petrify(
                node.as_ref().borrow().get_left_child(),
                x - node.as_ref().borrow().offset,
                max_depth - 1,
            );
            Self::petrify(
                node.as_ref().borrow().get_right_child(),
                x + node.as_ref().borrow().offset,
                max_depth - 1,
            );
        }
    }

    pub fn get_children(&self) -> (Link, Link) {
        (self.get_left_child(), self.get_right_child())
    }

    pub fn get_left_child(&self) -> Link {
        self.left_child.clone()
    }

    pub fn get_right_child(&self) -> Link {
        self.right_child.clone()
    }

    pub fn is_leaf(&self) -> bool {
        self.left_child.is_none() && self.right_child.is_none()
    }

    pub fn height(root: &Link) -> i32 {
        match root {
            Some(node) => {
                let node_ref = node.as_ref().borrow();
                let left_height = Self::height(&node_ref.left_child);
                let right_height = Self::height(&node_ref.right_child);
                1 + std::cmp::max(left_height, right_height)
            }
            None => 0,
        }
    }

    pub fn get_child_names(&self) -> (String, String) {
        if self.is_leaf() {
            return (String::from(""), String::from(""));
        }

        return (
            self.get_left_child()
                .as_ref()
                .unwrap()
                .as_ref()
                .borrow()
                .get_name(),
            self.get_right_child()
                .as_ref()
                .unwrap()
                .as_ref()
                .borrow()
                .get_name(),
        );
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_id(&self) -> i32 {
        i32::from_str_radix(self.name.as_str(), 16).unwrap_or(-1)
    }
    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }
    pub fn depth(&self) -> i32 {
        self.y as i32
    }
}
