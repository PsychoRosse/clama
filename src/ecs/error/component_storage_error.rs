use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComponentStorageError {
    #[error("Component storage type mismatch")]
    TypeMismatch,
}
