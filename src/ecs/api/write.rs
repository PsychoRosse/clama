use crate::{Component, QueryAccess, WorldCell, WriteStorage};

use std::marker::PhantomData;

#[derive(Default, Clone, Copy)]
pub struct Write<C: Component>(PhantomData<fn() -> C>);

impl<C: Component> QueryAccess for Write<C> {
    type Storage<'w> = WriteStorage<'w, C>;

    fn fetch<'w>(world: WorldCell<'w>) -> Self::Storage<'w> {
        WriteStorage::new(world.storage_mut::<C>(), world.change_tick())
    }
}
