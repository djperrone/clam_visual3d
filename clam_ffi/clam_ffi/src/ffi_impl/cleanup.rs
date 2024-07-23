// use crate::debug;
use crate::ffi_impl::string_ffi::StringFFI;

pub trait Cleanup {
    fn free(&mut self);
}

// impl Cleanup for ClusterIDs {
//     fn free(&mut self) {
//         self.id.free();
//         self.left_id.free();
//         self.right_id.free();
//     }
// }
// impl Cleanup for ClusterData {
//     fn free(&mut self) {
//         self.id.free();
//         self.message.free();
//     }
// }

impl Cleanup for StringFFI {
    fn free(&mut self) {
        self.free_data();
    }
}
