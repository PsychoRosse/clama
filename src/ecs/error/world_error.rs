use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorldError {
    #[error("Resource not registered")]
    ResourceNotRegistered,
    #[error("Event not registered")]
    EventNotRegistered,
}
