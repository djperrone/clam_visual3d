#![allow(dead_code)]
#![allow(unused_variables)]

use crate::utils::error::FFIError;
use ndarray::prelude::*;

pub static ANOMALY_DATASETS: &[&str] = &[
    "annthyroid",  // 0
    "arrhythmia",  // 1
    "breastw",     // 2
    "cardio",      // 3
    "cover",       // 4
    "glass",       // 5
    "http",        // 6
    "ionosphere",  // 7
    "lympho",      // 8
    "mammography", // 9
    "mnist",       // 10
    "musk",        // 11
    "optdigits",   // 12
    "pendigits",   // 13
    "pima",        // 14
    "satellite",   // 15
    "satimage-2",  // 16
    "shuttle",     // 17
    "smtp",        // 18
    "thyroid",     // 19
    "vertebral",   // 20
    "vowels",      // 21
    "wbc",         // 22
    "wine",        // 23
];

pub fn read_anomaly_data(
    name: &str,
    normalized: bool,
) -> Result<(Vec<Vec<f32>>, Vec<u8>), FFIError> {
    let mut data_dir = std::env::current_dir().unwrap();
    data_dir.pop();
    data_dir.push("data");
    data_dir.push("anomaly_data");
    data_dir.push("preprocessed");
    if !data_dir.exists() {
        let p = data_dir.to_str().unwrap_or("path is empty");
        return Err(FFIError::PathNotFound);
    }

    let features = {
        let mut path = data_dir.clone();
        path.push(if normalized {
            format!("{name}_features_normalized.npy")
        } else {
            format!("{name}_features.npy")
        });

        if !path.exists() {
            // debug!("path {:?} - {}", path, path.exists());
            let p = path.to_str().unwrap_or("path is empty");
            return Err(FFIError::PathNotFound);
        }

        let features: Array2<f32> =
            ndarray_npy::read_npy(&path).map_err(|error| FFIError::PathNotFound)?;

        features.outer_iter().map(|row| row.to_vec()).collect()
    };

    let scores = {
        let mut path = data_dir.clone();
        path.push(format!("{name}_scores.npy"));

        if !path.exists() {
            return Err(FFIError::PathNotFound);
        }

        let features: Array1<u8> =
            ndarray_npy::read_npy(&path).map_err(|error| FFIError::PathNotFound)?;

        features.to_vec()
    };

    Ok((features, scores))
}
