use crate::{Component, QueryFilter, SparseSet, Tick, WorldCell};

use std::marker::PhantomData;

pub struct Added<C>(PhantomData<fn() -> C>);

impl<C> Default for Added<C> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<C: Component> QueryFilter for Added<C> {
    type Storage<'w> = Option<&'w SparseSet<C>>;

    fn fetch<'w>(cell: WorldCell<'w>) -> Self::Storage<'w> {
        cell.storage::<C>()
    }

    fn matches(storage: &Self::Storage<'_>, entity: u32, last_run: Tick, this_run: Tick) -> bool {
        let Some(set) = storage else {
            return false;
        };

        set.added_tick(entity)
            .is_some_and(|tick| tick.is_newer_than(last_run, this_run))
    }
}
