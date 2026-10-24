use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum QueryError {
    #[error("Storage missing")]
    StorageMissing,
}
