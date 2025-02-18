pub mod api;
mod datamodel;
mod impl_unique;
mod impl_concat;
mod process;
pub use api::PostProcessReader;
pub use impl_unique::PostProcess;
pub use impl_concat::ConcatPostPrcess;
// use datamodel::{
//     get_n_export_real, read_avg_model_properties, read_model_properties, vec_to_array_view2,
//     vec_to_array_view3, Dim,
// };
// use datamodel::{make_histogram, Results};
use ndarray::{Array1, Array2, Array3, ArrayView3, Axis};
// use process::{spatial_average_concentration, Histogram};

/// `Phase` enum represents different states or phases of a substance.
#[derive(Clone, PartialEq, Copy)]
pub enum Phase {
    Liquid,
    Gas,
}

pub use datamodel::Weight;
