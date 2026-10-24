use std::any::Any;

pub(crate) trait ComponentStorage: Any + Send + Sync {
    fn remove_from_entity(&mut self, entity: u32);

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}
