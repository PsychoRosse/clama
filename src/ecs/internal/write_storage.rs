use crate::{Component, QueryError, QueryStorage, SparseSet, Tick};

use std::{marker::PhantomData, ptr::NonNull};

pub struct WriteStorage<'w, C: Component> {
    storage: Option<NonNull<SparseSet<C>>>,
    marker: PhantomData<&'w mut SparseSet<C>>,
    tick: Tick,
}

impl<'w, C: Component> WriteStorage<'w, C> {
    pub fn new(storage: Option<&'w mut SparseSet<C>>, tick: Tick) -> Self {
        Self {
            storage: storage.map(NonNull::from),
            marker: PhantomData,
            tick,
        }
    }
}

impl<'w, C: Component> QueryStorage<'w> for WriteStorage<'w, C> {
    type Item = &'w mut C;

    fn len(&self) -> usize {
        self.storage
            .map(|storage| unsafe { storage.as_ref().dense().len() })
            .unwrap_or(0)
    }

    fn entity(&self, dense: usize) -> u32 {
        unsafe {
            *self
                .storage
                .unwrap_or_else(|| panic!("{}", QueryError::StorageMissing))
                .as_ref()
                .indices()
                .get_unchecked(dense)
        }
    }

    fn contains(&self, entity: u32) -> bool {
        unsafe {
            self.storage
                .is_some_and(|storage| storage.as_ref().contains(entity))
        }
    }

    fn get(&mut self, dense: usize) -> Option<Self::Item> {
        let mut storage = self.storage?;

        unsafe {
            storage.as_mut().mark_changed_dense(dense, self.tick);

            Some(&mut *storage.as_mut().dense_mut().get_unchecked_mut(dense))
        }
    }

    fn get_by_entity(&mut self, entity: u32) -> Option<Self::Item> {
        let mut storage = self.storage?;

        unsafe {
            let dense = storage.as_ref().dense_index(entity)?;

            storage.as_mut().mark_changed_dense(dense, self.tick);

            Some(&mut *storage.as_mut().dense_mut().get_unchecked_mut(dense))
        }
    }
}
