use crate::{Resource, SystemParam, Tick, WorldCell};

use std::ops::Deref;

pub struct Res<'w, R> {
    value: &'w R,
}

impl<'w, R> Res<'w, R> {
    pub fn new(value: &'w R) -> Self {
        Self { value }
    }
}

impl<'w, R> Deref for Res<'w, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<R: Resource> SystemParam for Res<'_, R> {
    type Item<'w> = Res<'w, R>;

    fn fetch<'w>(cell: WorldCell<'w>, _: Tick, _: Tick) -> Self::Item<'w> {
        Res::new(cell.get_res_now::<R>())
    }
}

impl<R: Resource> SystemParam for Option<Res<'_, R>> {
    type Item<'w> = Option<Res<'w, R>>;

    fn fetch<'w>(cell: WorldCell<'w>, _: Tick, _: Tick) -> Self::Item<'w> {
        cell.get_res::<R>().map(Res::new)
    }
}
