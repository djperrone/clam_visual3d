use std::ffi::{c_char, CStr};

use super::error::FFIError;

pub fn label_colors() -> Vec<glam::Vec3> {
    let denom: f32 = 255.0;

    vec![
        glam::vec3(34.0 / denom, 199.0 / denom, 148.0 / denom),
        glam::vec3(249.0 / denom, 0.0 / denom, 152.0 / denom),
        glam::vec3(189.0 / denom, 129.0 / denom, 250.0 / denom),
        glam::vec3(0.0 / denom, 231.0 / denom, 250.0 / denom),
        glam::vec3(227.0 / denom, 250.0 / denom, 0.0 / denom),
        glam::vec3(250.0 / denom, 133.0 / denom, 0.0 / denom),
        glam::vec3(144.0 / denom, 0.0 / denom, 158.0 / denom),
        glam::vec3(245.0 / denom, 162.0 / denom, 218.0 / denom),
        glam::vec3(131.0 / denom, 44.0 / denom, 64.0 / denom),
        glam::vec3(210.0 / denom, 250.0 / denom, 198.0 / denom),
    ]
}

// fn parse_cluster_id(cluster_id: String) -> Result<(usize, usize), FFIError> {
//     let mut parts = cluster_id.split('-');

//     if let (Some(offset_str), Some(cardinality_str)) = (parts.next(), parts.next()) {
//         if let (Ok(offset), Ok(cardinality)) = (
//             offset_str.parse::<usize>(),
//             cardinality_str.parse::<usize>(),
//         ) {
//             return Ok((offset, cardinality));
//         }
//     }
//     Err(FFIError::InvalidStringPassed)
// }

// Function to parse the string returned by the name function into two integers
pub fn parse_cluster_name(name: &str) -> Result<(usize, usize), FFIError> {
    // Split the string into substrings based on the hyphen ("-")
    let parts: Vec<&str> = name.split('-').collect();

    // Ensure we have exactly two parts
    if parts.len() == 2 {
        // Parse the substrings into integers and return them as a tuple
        if let (Ok(offset), Ok(cardinality)) =
            (parts[0].parse::<usize>(), parts[1].parse::<usize>())
        {
            return Ok((offset, cardinality));
        }
    }
    Err(FFIError::InvalidStringPassed)
}

fn parse_cluster_id_raw(cluster_id: *const c_char) -> Result<(usize, usize), FFIError> {
    let cluster_id = c_char_to_string(cluster_id);
    parse_cluster_name(cluster_id.as_str())
}
#[no_mangle]
pub fn c_char_to_string(s: *const c_char) -> String {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };
    let r_str = c_str.to_str().unwrap();

    String::from(r_str)
}

pub unsafe fn csharp_to_rust_utf8(utf8_str: *const u8, utf8_len: i32) -> Result<String, FFIError> {
    let slice = std::slice::from_raw_parts(utf8_str, utf8_len as usize);
    match String::from_utf8(slice.to_vec()) {
        Ok(str) => Ok(str),
        Err(_) => Err(FFIError::InvalidStringPassed),
    }
}
