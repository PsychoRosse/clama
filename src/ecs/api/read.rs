use crate::{Component, QueryAccess, ReadStorage, WorldCell};

use std::marker::PhantomData;

pub struct Read<C: Component>(PhantomData<fn() -> C>);

impl<C: Component> QueryAccess for Read<C> {
    type Storage<'w> = ReadStorage<'w, C>;

    fn fetch<'w>(world: WorldCell<'w>) -> Self::Storage<'w> {
        ReadStorage::from(world.storage::<C>())
    }
}
