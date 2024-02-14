use crate::graph;
use crate::utils::types::Clusterf32;
use crate::ClusterData;

// pub struct NodeNDim<const N: usize> {
//     position: [f32; N],
//     friction: f32,
//     max_speed: f32,
//     velocity: [f32; N],
//     acceleration: [f32; N],
//     mass: f32,
// }

pub struct PhysicsNode {
    position: glam::Vec3,
    friction: f32,
    max_speed: f32,
    velocity: glam::Vec3,
    acceleration: glam::Vec3,
    mass: f32,
}

impl PhysicsNode {
    pub fn new(node_data: &ClusterData, cluster: &Clusterf32) -> Self {
        PhysicsNode {
            position: node_data.pos,
            friction: 0.98,
            max_speed: 5.,
            velocity: glam::Vec3::new(0., 0., 0.),
            acceleration: glam::Vec3::new(0., 0., 0.),
            mass: cluster.cardinality() as f32,
        }
    }

    pub fn mass(&self) -> f32 {
        self.mass
    }

    // F = M * A
    //updates acceleration of node
    pub fn accelerate(&mut self, force: glam::Vec3) {
        // A = F / M
        self.acceleration += force / self.mass();
    }

    pub fn get_position(&self) -> glam::Vec3 {
        self.position
    }

    //applies acceleration to velocity, applies velocity of node's position then updates sphere object on canvas
    pub fn update_position(&mut self) {
        self.velocity += self.acceleration;
        self.velocity *= self.friction; //reduce velocity by applying friction

        //if current velocity > max_speed, set velocity to max speed (to prevent extreme rubber banding in some graphs)
        if graph::helpers::get_magnitude(self.velocity) > self.max_speed {
            self.velocity = graph::helpers::set_magnitude(self.velocity, self.max_speed);
        }

        self.position += self.velocity;
        self.acceleration.x = 0.;
        self.acceleration.y = 0.;
        self.acceleration.z = 0.;
    }
}
