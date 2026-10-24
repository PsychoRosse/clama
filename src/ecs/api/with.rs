use crate::{Component, QueryFilter, SparseSet, Tick, WorldCell};

use std::marker::PhantomData;

pub struct With<C>(PhantomData<fn() -> C>);

impl<C: Component> QueryFilter for With<C> {
    type Storage<'w> = Option<&'w SparseSet<C>>;

    fn fetch<'w>(cell: WorldCell<'w>) -> Self::Storage<'w> {
        cell.storage::<C>()
    }

    fn matches(storage: &Self::Storage<'_>, entity: u32, _: Tick, _: Tick) -> bool {
        storage.is_some_and(|storage| storage.contains(entity))
    }
}
