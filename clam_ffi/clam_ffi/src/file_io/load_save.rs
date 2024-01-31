use crate::debug;
use crate::utils::error::FFIError;
use crate::utils::helpers;
use crate::utils::types::InHandlePtr;
use std::path::Path;

// pub fn load_single_f32(
//     path: &String,
// ) -> Result<Cakes<Vec<f32>, f32, VecDataset<Vec<f32>, f32, bool>>, String> {
//     return Cakes::<Vec<f32>, f32, VecDataset<_, _, _>>::load(
//         Path::new(path),
//         utils::distances::euclidean,
//         false,
//     );
// }
pub unsafe fn save_cakes_single_impl(
    ptr: InHandlePtr,
    file_name: *const u8,
    name_len: i32,
) -> FFIError {
    return if let Some(handle) = ptr {
        let path = match helpers::csharp_to_rust_utf8(file_name, name_len) {
            Ok(data_name) => data_name,
            Err(e) => {
                debug!("save cakes single error: {:?}", e);
                return e;
            }
        };

        if let Some(tree) = handle.tree() {
            let p = Path::new(&path);
            match tree.save(p) {
                Ok(_) => FFIError::Ok,
                Err(e) => {
                    debug!("save cakes single error: {:?}", e);
                    FFIError::SaveFailed
                }
            }
        } else {
            FFIError::HandleInitFailed
        }
    } else {
        FFIError::InvalidStringPassed
    };
}
