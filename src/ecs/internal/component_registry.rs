use crate::{Component, ComponentStorage, ComponentStorageError, SparseSet, Tick};

use {std::any::TypeId, std::collections::HashMap};

#[derive(Default)]
pub(crate) struct ComponentRegistry {
    storages: HashMap<TypeId, Box<dyn ComponentStorage>>,
}

impl ComponentRegistry {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            storages: HashMap::with_capacity(capacity),
        }
    }
}

impl ComponentRegistry {
    pub fn push<C: Component>(&mut self, entity: u32, component: C, tick: Tick) {
        self.storage_or_insert_mut::<C>()
            .insert_at(entity, component, tick);
    }

    pub fn remove<C: Component>(&mut self, entity: u32) -> Option<C> {
        self.get_storage_mut::<C>()?.remove(entity)
    }

    pub fn remove_from_entity(&mut self, entity: u32) {
        for storage in self.storages.values_mut() {
            storage.remove_from_entity(entity);
        }
    }

    pub fn get<C: Component>(&self, entity: u32) -> Option<&C> {
        self.get_storage::<C>()?.get(entity)
    }

    #[allow(dead_code)]
    pub fn get_mut<C: Component>(&mut self, entity: u32) -> Option<&mut C> {
        self.get_storage_mut::<C>()?.get_mut(entity)
    }

    #[allow(dead_code)]
    pub fn contains<C: Component>(&self, entity: u32) -> bool {
        self.get_storage::<C>()
            .is_some_and(|storage| storage.contains(entity))
    }

    #[allow(dead_code)]
    pub fn contains_storage<C: Component>(&self) -> bool {
        self.storages.contains_key(&TypeId::of::<C>())
    }

    pub fn get_storage<C: Component>(&self) -> Option<&SparseSet<C>> {
        self.storages
            .get(&TypeId::of::<C>())
            .and_then(|storage| storage.as_any().downcast_ref::<SparseSet<C>>())
    }

    pub fn get_storage_mut<C: Component>(&mut self) -> Option<&mut SparseSet<C>> {
        self.storages
            .get_mut(&TypeId::of::<C>())
            .and_then(|storage| storage.as_any_mut().downcast_mut::<SparseSet<C>>())
    }

    pub fn storage_or_insert_mut<C: Component>(&mut self) -> &mut SparseSet<C> {
        let id = TypeId::of::<C>();

        self.storages
            .entry(id)
            .or_insert_with(|| Box::new(SparseSet::<C>::default()))
            .as_any_mut()
            .downcast_mut::<SparseSet<C>>()
            .unwrap_or_else(|| panic!("{}", ComponentStorageError::TypeMismatch))
    }
}
