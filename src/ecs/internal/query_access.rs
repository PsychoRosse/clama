use crate::{QueryStorage, WorldCell};

pub trait QueryAccess {
    type Storage<'w>: QueryStorage<'w>;

    fn fetch<'w>(world: WorldCell<'w>) -> Self::Storage<'w>;
}
