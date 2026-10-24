use crate::{
    Bundle, Component, ComponentRegistry, Entity, EntityAllocator, Event, EventBuffer,
    EventRegistry, Resource, ResourceRegistry, SparseSet, Tick, WorldCell, WorldError,
};

pub struct World {
    allocator: EntityAllocator,
    storages: ComponentRegistry,
    resources: ResourceRegistry,
    events: EventRegistry,
    tick: Tick,
}

impl Default for World {
    fn default() -> Self {
        Self {
            allocator: EntityAllocator::default(),
            storages: ComponentRegistry::default(),
            resources: ResourceRegistry::default(),
            events: EventRegistry::default(),
            tick: Tick::INITIAL,
        }
    }
}

impl World {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            allocator: EntityAllocator::with_capacity(capacity),
            storages: ComponentRegistry::with_capacity(capacity),
            resources: ResourceRegistry::with_capacity(capacity),
            events: EventRegistry::with_capacity(capacity),
            tick: Tick::INITIAL,
        }
    }
}

impl World {
    pub fn spawn<B: Bundle>(&mut self, bundle: B) -> Entity {
        let entity = self.allocator.create();
        bundle.insert(entity, WorldCell::from(self));
        entity
    }

    pub fn despawn(&mut self, entity: Entity) -> bool {
        if !self.allocator.destroy(entity) {
            return false;
        }

        self.storages.remove_from_entity(entity.index());

        true
    }

    pub fn with_component<C: Component>(&mut self, entity: Entity, component: C) {
        if !self.allocator.is_alive(entity) {
            return;
        }

        self.storages.push(entity.index(), component, self.tick);
    }
    pub fn remove_component<C: Component>(&mut self, entity: Entity) -> Option<C> {
        if !self.allocator.is_alive(entity) {
            return None;
        }

        self.storages.remove::<C>(entity.index())
    }

    pub fn component<C: Component>(&self, entity: Entity) -> Option<&C> {
        if !self.allocator.is_alive(entity) {
            return None;
        }

        self.storages.get(entity.index())
    }

    pub fn component_mut<C: Component>(&mut self, entity: Entity) -> Option<&mut C> {
        if !self.allocator.is_alive(entity) {
            return None;
        }

        let tick = self.tick;
        let storage = self.storages.get_storage_mut::<C>()?;

        storage.mark_changed(entity.index(), tick);
        storage.get_mut(entity.index())
    }

    pub fn change_tick(&self) -> Tick {
        self.tick
    }

    pub fn advance_change_tick(&mut self) {
        self.tick.increment();
    }

    pub(crate) fn allocator(&self) -> &EntityAllocator {
        &self.allocator
    }

    pub(crate) fn storage<C: Component>(&self) -> Option<&SparseSet<C>> {
        self.storages.get_storage::<C>()
    }

    pub(crate) fn storage_mut<C: Component>(&mut self) -> Option<&mut SparseSet<C>> {
        self.storages.get_storage_mut::<C>()
    }
}

impl World {
    pub fn with_res<R: Resource>(&mut self, resource: R) -> &mut Self {
        self.resources.push(resource);

        self
    }

    pub fn remove_res<R: Resource>(&mut self) -> Option<R> {
        self.resources.remove::<R>()
    }

    pub fn get_res<R: Resource>(&self) -> Option<&R> {
        self.resources.get::<R>()
    }

    pub fn get_res_mut<R: Resource>(&mut self) -> Option<&mut R> {
        self.resources.get_mut::<R>()
    }

    pub fn get_res_now<R: Resource>(&self) -> &R {
        self.resources
            .get::<R>()
            .unwrap_or_else(|| panic!("{}", WorldError::ResourceNotRegistered))
    }

    pub fn get_res_mut_now<R: Resource>(&mut self) -> &mut R {
        self.resources
            .get_mut::<R>()
            .unwrap_or_else(|| panic!("{}", WorldError::ResourceNotRegistered))
    }

    pub fn contains_res<R: Resource>(&self) -> bool {
        self.resources.contains::<R>()
    }
}

impl World {
    pub fn with_event<E: Event>(&mut self, event: EventBuffer<E>) -> &mut Self {
        self.events.push(event);

        self
    }

    pub fn get_event<E: Event>(&self) -> Option<&EventBuffer<E>> {
        self.events.get::<E>()
    }

    pub fn get_mut_event<E: Event>(&mut self) -> Option<&mut EventBuffer<E>> {
        self.events.get_mut::<E>()
    }

    pub fn get_event_now<E: Event>(&self) -> &EventBuffer<E> {
        self.events
            .get::<E>()
            .unwrap_or_else(|| panic!("{}", WorldError::EventNotRegistered))
    }

    pub fn get_event_mut_now<E: Event>(&mut self) -> &mut EventBuffer<E> {
        self.events
            .get_mut::<E>()
            .unwrap_or_else(|| panic!("{}", WorldError::EventNotRegistered))
    }

    pub fn update_events(&mut self) {
        self.events.update();
    }
}
