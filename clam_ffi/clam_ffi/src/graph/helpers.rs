pub fn get_magnitude(vector: glam::Vec3) -> f32 {
    vector.length()
}

pub fn set_magnitude(mut vector: glam::Vec3, new_mag: f32) -> glam::Vec3 {
    let old_mag = vector.length();

    if old_mag.abs() > f32::EPSILON {
        let ratio: f32 = new_mag / old_mag;
        vector *= ratio;
    } else {
        // Handle the case where the original magnitude is very close to zero
        // You can choose to return a default vector or handle it differently based on your requirements
        // Here, we return a zero vector as an example
        vector = glam::Vec3::ZERO;
    }

    vector
}
