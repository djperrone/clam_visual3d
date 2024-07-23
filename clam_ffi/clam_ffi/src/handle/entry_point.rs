use crate::handle::handle::Handle;
use crate::utils::distances::DistanceMetric;
use crate::utils::helpers;
use crate::utils::types::{InHandlePtr, OutHandlePtr};

use crate::utils::error::FFIError;

use crate::debug;
use crate::ffi_impl::tree_startup_data_ffi::TreeStartupDataFFI;

pub unsafe fn shutdown_clam_impl(context_ptr: OutHandlePtr) -> FFIError {
    if let Some(handle) = context_ptr {
        let _ = Box::from_raw(*handle);
        FFIError::Ok
    } else {
        debug!("shtudown clam handle not valid?");
        FFIError::NullPointerPassed
    }
}

pub unsafe fn init_clam_struct_impl(
    ptr: OutHandlePtr,
    data: Option<&TreeStartupDataFFI>,
) -> FFIError {
    let data = match data {
        Some(data) => data,
        None => {
            debug!("data is null");
            return FFIError::NullPointerPassed;
        }
    };
    let data_name = match data.data_name.as_string() {
        Ok(data_name) => data_name,
        Err(e) => {
            debug!("{:?}", e);
            return FFIError::InvalidStringPassed;
        }
    };

    match Handle::new(
        &data_name,
        data.cardinality as usize,
        data.distance_metric,
        data.is_expensive,
    ) {
        Ok(handle) => {
            if let Some(out_handle) = ptr {
                *out_handle = Box::into_raw(Box::new(handle));
            }
            debug!("built clam tree for {}", data_name);
            FFIError::Ok
        }
        Err(e) => {
            debug!("{:?}", e);
            FFIError::HandleInitFailed
        }
    }
}

pub unsafe fn init_clam_impl(
    ptr: OutHandlePtr,
    data_name: *const u8,
    name_len: i32,
    cardinality: usize,
    distance_metric: DistanceMetric,
) -> FFIError {
    let data_name = match helpers::csharp_to_rust_utf8(data_name, name_len) {
        Ok(data_name) => data_name,
        Err(e) => {
            debug!("{:?}", e);
            return FFIError::InvalidStringPassed;
        }
    };

    match Handle::new(&data_name, cardinality, distance_metric, false) {
        Ok(handle) => {
            if let Some(out_handle) = ptr {
                *out_handle = Box::into_raw(Box::new(handle));
            }

            debug!("built clam tree for {}", data_name);
            FFIError::Ok
        }
        Err(e) => {
            debug!("{:?}", e);
            FFIError::HandleInitFailed
        }
    }
}

pub unsafe fn load_cakes_struct_impl(
    ptr: OutHandlePtr,
    data: Option<&TreeStartupDataFFI>,
) -> FFIError {
    let data = match data {
        Some(data) => data,
        None => {
            debug!("data is null");
            return FFIError::NullPointerPassed;
        }
    };

    match Handle::load_struct(data) {
        Ok(handle) => {
            if let Some(out_handle) = ptr {
                *out_handle = Box::into_raw(Box::new(handle));
            }

            debug!(
                "built clam tree for {}",
                data.data_name.as_string().unwrap()
            );
            FFIError::Ok
        }
        Err(e) => {
            debug!("{:?}", e);
            FFIError::HandleInitFailed
        }
    }
}
