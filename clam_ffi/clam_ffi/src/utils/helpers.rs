use std::ffi::{c_char, CStr, CString};

use super::error::FFIError;

// pub fn alloc_to_c_char(str: String) -> *mut c_char {
//     let str = CString::new(str).unwrap();
//     str.into_raw()
// }

// pub fn free_c_char(str: *mut c_char) {
//     if !str.is_null() {
//         unsafe { CString::from_raw(str) };
//     } else {
//         debug!("tried to free null string");
//     }
// }

// pub fn free_string(str: *mut u8) {
//     if !str.is_null() {
//         unsafe { let _ = CString::from_raw(str as *mut i8); };
//     } else {
//         debug!("tried to free null string");
//     }
// }

// start_node: *const c_char
// if !start_node.is_null() {
//     let c_str = unsafe {
//         // assert!(!start_node.is_null());

//         CStr::from_ptr(start_node)
//     };
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
    return Err(FFIError::InvalidStringPassed);
}

fn parse_cluster_id_raw(cluster_id: *const c_char) -> Result<(usize, usize), FFIError> {
    let cluster_id = c_char_to_string(cluster_id);
    return parse_cluster_id(cluster_id);
}
#[no_mangle]
pub fn c_char_to_string(s: *const c_char) -> String {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };
    // debug!("cstr testing {:?}", c_str);
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

pub fn hex_to_binary(hex_string: String) -> String {
    let mut binary_string = String::new();

    for hex_char in hex_string.chars() {
        match hex_char {
            '0' => binary_string.push_str("0000"),
            '1' => binary_string.push_str("0001"),
            '2' => binary_string.push_str("0010"),
            '3' => binary_string.push_str("0011"),
            '4' => binary_string.push_str("0100"),
            '5' => binary_string.push_str("0101"),
            '6' => binary_string.push_str("0110"),
            '7' => binary_string.push_str("0111"),
            '8' => binary_string.push_str("1000"),
            '9' => binary_string.push_str("1001"),
            'A' | 'a' => binary_string.push_str("1010"),
            'B' | 'b' => binary_string.push_str("1011"),
            'C' | 'c' => binary_string.push_str("1100"),
            'D' | 'd' => binary_string.push_str("1101"),
            'E' | 'e' => binary_string.push_str("1110"),
            'F' | 'f' => binary_string.push_str("1111"),
            _ => {}
        }
    }

    binary_string
}
