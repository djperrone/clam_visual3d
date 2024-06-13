use abd_clam::Cluster;

use super::string_ffi::StringFFI;
use crate::utils::types::Vertexf32;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ClusterIDs {
    pub id: StringFFI,
    pub left_id: StringFFI,
    pub right_id: StringFFI,
}

impl ClusterIDs {
    pub unsafe fn get_id(&self) -> String {
        self.id.as_string().unwrap()
    }

    pub unsafe fn get_ffi_id(&self) -> &StringFFI {
        &self.id
    }

    pub fn from_clam(node: &Vertexf32) -> Self {
        let (left_id, right_id) = {
            if let Some([left, right]) = node.children() {
                (left.name(), right.name())
            } else {
                ("None".to_string(), "None".to_string())
            }
        };

        ClusterIDs {
            id: (StringFFI::new(node.name())),
            left_id: StringFFI::new(left_id),
            right_id: StringFFI::new(right_id),
        }
    }

    pub fn free_ids(&mut self) {
        self.id.free_data();
        self.left_id.free_data();
        self.right_id.free_data();
    }
}
