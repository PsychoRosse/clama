use crate::{Entity, EntityAllocator, QueryStorage};

pub struct EntityStorage {
    entities: Vec<Entity>,
}

impl EntityStorage {
    pub(crate) fn new(allocator: &EntityAllocator) -> Self {
        Self {
            entities: allocator.alive_entities(),
        }
    }
}

impl<'w> QueryStorage<'w> for EntityStorage {
    type Item = Entity;

    fn len(&self) -> usize {
        self.entities.len()
    }

    fn entity(&self, dense: usize) -> u32 {
        self.entities[dense].index()
    }

    fn contains(&self, entity: u32) -> bool {
        self.index_of(entity).is_some()
    }

    fn get(&mut self, dense: usize) -> Option<Self::Item> {
        self.entities.get(dense).copied()
    }

    fn get_by_entity(&mut self, entity: u32) -> Option<Self::Item> {
        self.index_of(entity).map(|dense| self.entities[dense])
    }
}

impl EntityStorage {
    fn index_of(&self, index: u32) -> Option<usize> {
        self.entities
            .binary_search_by_key(&index, |entity| entity.index())
            .ok()
    }
}
