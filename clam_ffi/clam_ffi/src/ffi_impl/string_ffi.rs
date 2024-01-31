use std::{
    ffi::{c_char, CStr, CString},
    ptr::null_mut,
};

use crate::utils::error::FFIError;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct StringFFI {
    pub data: *mut u8,
    pub len: i32,
}

impl StringFFI {
    pub fn new(str: String) -> Self {
        StringFFI {
            len: str.len() as i32,
            data: CString::new(str).unwrap().into_raw() as *mut u8,
        }
    }

    pub fn default() -> Self {
        StringFFI {
            len: 0i32,
            data: null_mut(),
        }
    }

    pub fn as_string(&self) -> Result<String, FFIError> {
        unsafe {
            if self.data.is_null() {
                return Err(FFIError::NullPointerPassed);
            }
            let slice = std::slice::from_raw_parts(self.data, self.len as usize);
            match String::from_utf8(slice.to_vec()) {
                Ok(str) => Ok(str),
                Err(_) => Err(FFIError::InvalidStringPassed),
            }
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.data
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_null()
    }

    pub fn free_data(&mut self) {
        unsafe {
            if !self.data.is_null() {
                {
                    drop(CString::from_raw(self.data as *mut i8));
                    self.len = 0;
                    self.data = null_mut();
                };
            }
        }
    }

    pub fn c_char_to_string(s: *const c_char) -> String {
        let c_str = unsafe {
            assert!(!s.is_null());
            CStr::from_ptr(s)
        };
        let r_str = c_str.to_str().unwrap();
        String::from(r_str)
    }
}
