use crate::{Bundle, Entity, Resource, SystemParam, Tick, WorldCell};

#[derive(Clone, Copy)]
pub struct Commands<'c> {
    world: WorldCell<'c>,
}

impl<'c> Commands<'c> {
    pub fn new(world: WorldCell<'c>) -> Commands<'c> {
        Self { world }
    }
}

impl<'c> Commands<'c> {
    pub fn spawn<B: Bundle>(&self, bundle: B) -> Entity {
        self.world.spawn(bundle)
    }

    pub fn despawn(&self, entity: Entity) -> bool {
        self.world.despawn(entity)
    }

    pub fn with_res<R: Resource>(&self, resource: R) {
        self.world.with_res(resource);
    }
}

impl SystemParam for Commands<'_> {
    type Item<'c> = Commands<'c>;

    fn fetch<'c>(world: WorldCell<'c>, _: Tick, _: Tick) -> Self::Item<'c> {
        Commands::new(world)
    }
}
