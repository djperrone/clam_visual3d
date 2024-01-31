use std::ffi::{c_char, CStr};

use super::error::FFIError;

fn parse_cluster_id(cluster_id: String) -> Result<(usize, usize), FFIError> {
    let mut parts = cluster_id.split('-');

    if let (Some(offset_str), Some(cardinality_str)) = (parts.next(), parts.next()) {
        if let (Ok(offset), Ok(cardinality)) = (
            offset_str.parse::<usize>(),
            cardinality_str.parse::<usize>(),
        ) {
            return Ok((offset, cardinality));
        }
    }
    Err(FFIError::InvalidStringPassed)
}

fn parse_cluster_id_raw(cluster_id: *const c_char) -> Result<(usize, usize), FFIError> {
    let cluster_id = c_char_to_string(cluster_id);
    parse_cluster_id(cluster_id)
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
