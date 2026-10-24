use crate::{Event, EventBuffer, IntoSystem, Plugin, Resource, Stage};

pub trait Registry {
    fn with_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self;

    fn with_res<R: Resource>(&mut self, resource: R) -> &mut Self;

    fn with_event<E: Event>(&mut self, event: EventBuffer<E>) -> &mut Self;

    fn with_system<M, S>(&mut self, stage: Stage, system: S) -> &mut Self
    where
        S: IntoSystem<M> + 'static;
}
