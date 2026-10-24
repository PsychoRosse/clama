use crate::{Component, ComponentStorage, EntityError, Tick};

use std::{any::Any, mem::replace};

pub struct SparseSet<C> {
    sparse: Vec<u32>,
    indices: Vec<u32>,
    dense: Vec<C>,
    added: Vec<Tick>,
    changed: Vec<Tick>,
}

impl<C> SparseSet<C> {
    pub const INVALID_DENSE_INDEX: u32 = u32::MAX;
}

impl<C> Default for SparseSet<C> {
    fn default() -> Self {
        Self {
            sparse: Vec::new(),
            indices: Vec::new(),
            dense: Vec::new(),
            added: Vec::new(),
            changed: Vec::new(),
        }
    }
}

impl<C> SparseSet<C> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            sparse: Vec::with_capacity(capacity),
            indices: Vec::with_capacity(capacity),
            dense: Vec::with_capacity(capacity),
            added: Vec::with_capacity(capacity),
            changed: Vec::with_capacity(capacity),
        }
    }
}

impl<C> SparseSet<C> {
    pub fn contains(&self, entity: u32) -> bool {
        self.dense_index(entity).is_some()
    }

    pub fn get(&self, entity: u32) -> Option<&C> {
        let dense = self.dense_index(entity)?;
        Some(&self.dense[dense])
    }

    pub fn get_mut(&mut self, entity: u32) -> Option<&mut C> {
        let dense = self.dense_index(entity)?;
        Some(&mut self.dense[dense])
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    pub fn dense(&self) -> &[C] {
        &self.dense
    }

    pub fn dense_mut(&mut self) -> &mut [C] {
        &mut self.dense
    }

    pub fn insert(&mut self, entity: u32, component: C) -> Option<C> {
        self.insert_at(entity, component, Tick::ZERO)
    }

    pub fn insert_at(&mut self, entity: u32, component: C, tick: Tick) -> Option<C> {
        self.ensure_capacity(entity);

        if let Some(dense) = self.dense_index(entity) {
            self.changed[dense] = tick;
            return Some(replace(&mut self.dense[dense], component));
        }

        let dense = u32::try_from(self.dense.len())
            .unwrap_or_else(|_| panic!("{}", EntityError::GenerationOverflow));

        self.sparse[entity as usize] = dense;

        self.indices.push(entity);
        self.dense.push(component);
        self.added.push(tick);
        self.changed.push(tick);

        None
    }

    pub fn remove(&mut self, entity: u32) -> Option<C> {
        let dense = self.dense_index(entity)?;

        self.sparse[entity as usize] = Self::INVALID_DENSE_INDEX;

        let component = self.dense.swap_remove(dense);

        self.indices.swap_remove(dense);
        self.added.swap_remove(dense);
        self.changed.swap_remove(dense);

        if dense < self.indices.len() {
            let moved = self.indices[dense];
            self.sparse[moved as usize] = dense as u32;
        }

        Some(component)
    }

    pub fn dense_index(&self, entity: u32) -> Option<usize> {
        let dense = *self.sparse.get(entity as usize)?;

        if dense == Self::INVALID_DENSE_INDEX {
            return None;
        }

        let dense = dense as usize;

        (self.indices.get(dense).copied() == Some(entity)).then_some(dense)
    }

    pub fn ensure_capacity(&mut self, entity: u32) {
        let requied = entity as usize + 1;

        if self.sparse.len() < requied {
            self.sparse.resize(requied, Self::INVALID_DENSE_INDEX);
        }
    }
}

impl<C> SparseSet<C> {
    pub fn mark_changed_dense(&mut self, dense: usize, tick: Tick) {
        if let Some(changed_tick) = self.changed.get_mut(dense) {
            *changed_tick = tick;
        }
    }

    pub fn added_tick(&self, entity: u32) -> Option<Tick> {
        let dense = self.dense_index(entity)?;
        self.added.get(dense).copied()
    }

    pub fn changed_tick(&self, entity: u32) -> Option<Tick> {
        let dense = self.dense_index(entity)?;
        self.changed.get(dense).copied()
    }

    pub fn mark_changed(&mut self, entity: u32, tick: Tick) -> bool {
        let Some(dense) = self.dense_index(entity) else {
            return false;
        };

        self.changed[dense] = tick;
        true
    }
}

impl<C: Component> ComponentStorage for SparseSet<C> {
    fn remove_from_entity(&mut self, entity: u32) {
        let _ = self.remove(entity);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
