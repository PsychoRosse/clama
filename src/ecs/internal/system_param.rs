use crate::{Tick, WorldCell};

pub trait SystemParam {
    type Item<'w>;

    fn fetch<'w>(world: WorldCell<'w>, last_run: Tick, this_run: Tick) -> Self::Item<'w>;
}

impl SystemParam for () {
    type Item<'w> = ();

    fn fetch<'w>(_: WorldCell<'w>, _: Tick, _: Tick) -> Self::Item<'w> {}
}
