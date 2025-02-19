pub mod api;
mod datamodel;
mod impl_concat;
mod impl_unique;
mod process;
use core::fmt;

pub use api::PostProcessReader;
pub use datamodel::Weight;
pub use impl_concat::ConcatPostPrcess;
pub use impl_unique::PostProcess;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {

    #[error("Index {0:?} is Out of Range {1}")]
    OutOfRange(usize,usize),

    #[error("Property {0} does not exist in the selected dataset")]
    KeyError(String),

    #[error("Records quantity '{0}' does not exist in the selected dataset")]
    RecordsError(String),

    #[error("Datasetd shape mismatch")]
    ShapeError,

    #[error("Internal I/O error: {0}")]
    Io(#[from] hdf5::Error),

    #[error("Error: {0}")]
    Default(String),
}

// // Implement std::fmt::Display for AppError
// impl fmt::Display for AppError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Error {}",self.0)
//     }
// }

// // Implement std::fmt::Debug for AppError
// impl fmt::Debug for AppError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{{ file: {}, line: {},message: {} }}", file!(), line!(),self.0)
//     }
// }

/// `Phase` enum represents different states or phases of a substance.
#[derive(Clone, PartialEq, Copy)]
pub enum Phase {
    Liquid,
    Gas,
}

#[derive(Copy, Clone)]
pub enum Estimator {
    MonteCarlo,
    Weighted,
}
