use crate::{Resource, SystemParam, Tick, WorldCell};

use std::ops::{Deref, DerefMut};

pub struct ResMut<'w, T> {
    value: &'w mut T,
}
impl<'w, T> ResMut<'w, T> {
    pub fn new(value: &'w mut T) -> Self {
        Self { value }
    }
}

impl<'w, T> Deref for ResMut<'w, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'w, T> DerefMut for ResMut<'w, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}

impl<R: Resource> SystemParam for ResMut<'_, R> {
    type Item<'w> = ResMut<'w, R>;

    fn fetch<'w>(cell: WorldCell<'w>, _: Tick, _: Tick) -> Self::Item<'w> {
        ResMut::new(cell.get_res_mut_now::<R>())
    }
}

impl<R: Resource> SystemParam for Option<ResMut<'_, R>> {
    type Item<'w> = Option<ResMut<'w, R>>;

    fn fetch<'w>(cell: WorldCell<'w>, _: Tick, _: Tick) -> Self::Item<'w> {
        cell.get_res_mut::<R>().map(ResMut::new)
    }
}
