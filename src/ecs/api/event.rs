use std::any::{Any, type_name};

pub trait Event: Any + Send + Sync {
    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }
}
