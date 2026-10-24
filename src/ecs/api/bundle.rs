use crate::{Entity, WorldCell};

pub trait Bundle {
    fn insert(self, entity: Entity, world: WorldCell);
}

impl Bundle for () {
    fn insert(self, _: Entity, _: WorldCell) {}
}
