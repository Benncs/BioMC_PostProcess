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


