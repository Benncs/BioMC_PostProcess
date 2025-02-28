pub mod error;
pub mod api;
mod datamodel;
mod impl_concat;
mod impl_unique;
mod process;

pub use api::PostProcessReader;
pub use datamodel::Weight;
pub use impl_concat::ConcatPostPrcess;
pub use impl_unique::PostProcess;


