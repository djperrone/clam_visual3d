use std::cmp::Ordering;

use abd_clam::{graph::Vertex, Cluster};
use distances::Number;

pub struct FNN_Wrapper<'a, U: Number> {
    cluster: &'a Vertex<U>,
    distance: U,
}

impl<'a, U: Number> FNN_Wrapper<'a, U> {
    pub fn new(cluster: &'a Vertex<U>, distance: U) -> Self {
        FNN_Wrapper { cluster, distance }
    }
}

impl<'a, U: Number> Eq for FNN_Wrapper<'a, U> {}

// Implementing PartialEq based on name
impl<'a, U: Number> PartialEq for FNN_Wrapper<'a, U> {
    fn eq(&self, other: &Self) -> bool {
        self.cluster.name() == other.cluster.name()
    }
}

// Implementing PartialOrd based on distance
impl<'a, U: Number> PartialOrd for FNN_Wrapper<'a, U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

// Implementing Ord based on distance, with name as tie-breaker
impl<'a, U: Number> Ord for FNN_Wrapper<'a, U> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.distance.partial_cmp(&other.distance) {
            Some(Ordering::Equal) => self.cluster.name().cmp(&other.cluster.name()),
            Some(ordering) => ordering,
            None => Ordering::Equal, // Handle NaN case or any other undefined behavior
        }
    }
}

// impl<'a, U: Number> Ord for FNN_Wrapper<'a, U> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.distance
//             .partial_cmp(&other.distance)
//             .unwrap_or(Ordering::Equal)
//     }
// }