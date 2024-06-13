use abd_clam::{graph::{Graph, Vertex}, Tree, VecDataset};

use crate::handle::handle::Handle;

pub type OutHandlePtr<'a> = Option<&'a mut *mut Handle<'a>>;

pub type InHandlePtr<'a> = Option<&'a mut Handle<'a>>;

// pub type Clusterf32 = Cluster<f32>;
// pub type DataSetf32 = VecDataset<Vec<f32>, f32, u8>;
// pub type Treef32 = Tree<Vec<f32>, f32, DataSetf32>;
pub type Vertexf32 = Vertex<f32>;
pub type DataSetf32 = VecDataset<Vec<f32>, f32, u8>;
pub type Treef32 = Tree<Vec<f32>, f32, DataSetf32, Vertexf32>;
pub type Graphf32<'a> = Graph<'a, f32>;
// pub type Cakesf32 = Cakes<f32, f32, DataSet>;
