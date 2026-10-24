use crate::{
    Event, EventBuffer, IntoSystem, Plugin, Registry, Resource, Scheduler, Stage, WorldCell,
};

use std::any::TypeId;

type Resources = Box<dyn FnOnce(WorldCell)>;
type Events = Box<dyn FnOnce(WorldCell)>;
type Systems = Box<dyn FnOnce(&mut Scheduler)>;

#[derive(Default)]
pub struct PluginRegistry {
    plugins: Vec<TypeId>,
    resource: Vec<Resources>,
    events: Vec<Events>,
    systems: Vec<Systems>,
}

impl Registry for PluginRegistry {
    fn with_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        let id = TypeId::of::<P>();

        if self.plugins.contains(&id) {
            return self;
        }

        self.plugins.push(id);
        plugin.build(self);

        self
    }

    fn with_res<R: Resource>(&mut self, resource: R) -> &mut Self {
        self.resource.push(Box::new(move |cell| {
            cell.with_res(resource);
        }));

        self
    }

    fn with_event<E: Event>(&mut self, event: EventBuffer<E>) -> &mut Self {
        self.events.push(Box::new(move |cell| {
            cell.with_event(event);
        }));

        self
    }

    fn with_system<M, S>(&mut self, stage: Stage, system: S) -> &mut Self
    where
        S: IntoSystem<M> + 'static,
    {
        self.systems.push(Box::new(move |scheduler| {
            scheduler.add(stage, system);
        }));

        self
    }
}

impl PluginRegistry {
    #[inline]
    pub fn is_plugin_registered<P: Plugin>(&self) -> bool {
        self.plugins.contains(&TypeId::of::<P>())
    }

    pub fn build(&mut self, cell: WorldCell, scheduler: &mut Scheduler) {
        for resource in self.resource.drain(..) {
            resource(cell)
        }

        for event in self.events.drain(..) {
            event(cell)
        }

        for system in self.systems.drain(..) {
            system(scheduler)
        }
    }
}
