#![allow(dead_code)]
#![allow(unused_variables)]

use crate::utils::error::FFIError;
use distances;

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DistanceMetric {
    None,
    Euclidean,
    EuclideanSQ,
    Manhattan,
    L3Norm,
    L4Norm,
    Chebyshev,
    Cosine,
    Canberra,
    NeedlemanWunsch,
    Levenshtein,
}

//TODO: Make generic for strings as well
pub fn from_enum(metric: DistanceMetric) -> Result<fn(&Vec<f32>, &Vec<f32>) -> f32, FFIError> {
    match metric {
        DistanceMetric::Euclidean => Ok(euclidean),
        DistanceMetric::EuclideanSQ => Ok(euclidean_sq),
        DistanceMetric::Manhattan => Ok(manhattan),
        DistanceMetric::L3Norm => Ok(l3_norm),
        DistanceMetric::L4Norm => Ok(l4_norm),
        DistanceMetric::Chebyshev => Ok(chebyshev),
        DistanceMetric::Cosine => Ok(cosine),
        DistanceMetric::Canberra => Ok(canberra),

        // Handle unsupported or unimplemented metrics as an error
        _ => Err(FFIError::UnsupportedMetric),
    }
}

// lp_norms
pub fn euclidean(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    distances::vectors::euclidean(x, y)
}
pub fn euclidean_sq(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    distances::vectors::euclidean_sq(x, y)
}
pub fn manhattan(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    distances::vectors::manhattan(x, y)
}
pub fn l3_norm(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    distances::vectors::l3_norm(x, y)
}
pub fn l4_norm(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    distances::vectors::l3_norm(x, y)
}
pub fn chebyshev(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    distances::vectors::chebyshev(x, y)
}

pub fn cosine(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    distances::vectors::cosine(x, y)
}
pub fn canberra(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    distances::vectors::canberra(x, y)
}

//Needleman Wunsch
pub fn nw_distance(x: &str, y: &str) -> u32 {
    distances::strings::nw_distance(x, y)
}

//levenshtein
pub fn levenshtein(x: &str, y: &str) -> u32 {
    distances::strings::levenshtein(x, y)
}
