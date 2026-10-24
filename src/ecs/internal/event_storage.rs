use std::any::Any;

pub trait EventStorage: Any + Send + Sync {
    fn update(&mut self);

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}
