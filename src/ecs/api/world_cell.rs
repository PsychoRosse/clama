use crate::{
    Bundle, Component, Entity, EntityAllocator, Event, EventBuffer, Resource, SparseSet, Tick,
    World,
};

use std::{marker::PhantomData, ptr::NonNull};

#[derive(Clone, Copy)]
pub struct WorldCell<'w> {
    world: NonNull<World>,
    marker: PhantomData<&'w mut World>,
}

impl<'w> WorldCell<'w> {
    pub fn spawn<B: Bundle>(self, bundle: B) -> Entity {
        unsafe { (*self.world.as_ptr()).spawn(bundle) }
    }

    pub fn despawn(self, entity: Entity) -> bool {
        unsafe { (*self.world.as_ptr()).despawn(entity) }
    }

    pub fn with_component<C: Component>(self, entity: Entity, component: C) {
        unsafe {
            (*self.world.as_ptr()).with_component(entity, component);
        }
    }

    pub fn remove_component<C: Component>(self, entity: Entity) -> Option<C> {
        unsafe { (*self.world.as_ptr()).remove_component::<C>(entity) }
    }

    pub fn component<C: Component>(self, entity: Entity) -> Option<&'w C> {
        unsafe { (*self.world.as_ptr()).component::<C>(entity) }
    }

    pub fn component_mut<C: Component>(self, entity: Entity) -> Option<&'w mut C> {
        unsafe { (*self.world.as_ptr()).component_mut::<C>(entity) }
    }

    pub fn change_tick(self) -> Tick {
        unsafe { self.world.as_ref().change_tick() }
    }

    pub fn advance_change_tick(&self) {
        unsafe {
            (*self.world.as_ptr()).advance_change_tick();
        }
    }

    pub(crate) fn allocator(self) -> &'w EntityAllocator {
        unsafe { (*self.world.as_ptr()).allocator() }
    }

    pub(crate) fn storage<C: Component>(self) -> Option<&'w SparseSet<C>> {
        unsafe { (*self.world.as_ptr()).storage::<C>() }
    }

    pub(crate) fn storage_mut<C: Component>(self) -> Option<&'w mut SparseSet<C>> {
        unsafe { (*self.world.as_ptr()).storage_mut::<C>() }
    }
}

impl<'w> WorldCell<'w> {
    pub fn with_res<R: Resource>(self, resource: R) -> Self {
        unsafe { (*self.world.as_ptr()).with_res(resource).into() }
    }

    pub fn remove_res<R: Resource>(self) -> Option<R> {
        unsafe { (*self.world.as_ptr()).remove_res::<R>() }
    }

    pub fn get_res<R: Resource>(self) -> Option<&'w R> {
        unsafe { (*self.world.as_ptr()).get_res::<R>() }
    }

    pub fn get_res_mut<R: Resource>(self) -> Option<&'w mut R> {
        unsafe { (*self.world.as_ptr()).get_res_mut::<R>() }
    }

    pub fn get_res_now<R: Resource>(self) -> &'w R {
        unsafe { (*self.world.as_ptr()).get_res_now::<R>() }
    }

    pub fn get_res_mut_now<R: Resource>(self) -> &'w mut R {
        unsafe { (*self.world.as_ptr()).get_res_mut_now::<R>() }
    }

    pub fn contains_res<R: Resource>(self) -> bool {
        unsafe { (*self.world.as_ptr()).contains_res::<R>() }
    }
}

impl<'w> WorldCell<'w> {
    pub fn with_event<E: Event>(&self, event: EventBuffer<E>) -> Self {
        unsafe { (*self.world.as_ptr()).with_event(event).into() }
    }

    pub fn get_event<E: Event>(&self) -> Option<&'w EventBuffer<E>> {
        unsafe { (*self.world.as_ptr()).get_event::<E>() }
    }

    pub fn get_mut_event<E: Event>(&self) -> Option<&'w mut EventBuffer<E>> {
        unsafe { (*self.world.as_ptr()).get_mut_event::<E>() }
    }

    pub fn get_event_now<E: Event>(&self) -> &'w EventBuffer<E> {
        unsafe { (*self.world.as_ptr()).get_event_now::<E>() }
    }

    pub fn get_event_mut_now<E: Event>(&self) -> &'w mut EventBuffer<E> {
        unsafe { (*self.world.as_ptr()).get_event_mut_now::<E>() }
    }

    pub fn update(&self) {
        unsafe {
            (*self.world.as_ptr()).update_events();
        }
    }
}

impl<'w> From<&'w mut World> for WorldCell<'w> {
    fn from(value: &'w mut World) -> Self {
        Self {
            world: NonNull::from(value),
            marker: PhantomData,
        }
    }
}
