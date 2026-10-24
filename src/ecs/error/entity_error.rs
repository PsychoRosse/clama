use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntityError {
    #[error("Entity generation overflow")]
    GenerationOverflow,
}
