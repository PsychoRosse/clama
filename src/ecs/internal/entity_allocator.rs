use crate::{Entity, EntityError};

#[derive(Default)]
pub(crate) struct EntityAllocator {
    free: Vec<u32>,
    generation: Vec<u32>,
}

impl EntityAllocator {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            free: Vec::with_capacity(capacity),
            generation: Vec::with_capacity(capacity),
        }
    }
}

impl EntityAllocator {
    pub fn create(&mut self) -> Entity {
        if let Some(id) = self.free.pop() {
            let generation = self.generation[id as usize];

            Entity::new(id, generation)
        } else {
            let id = self.generation.len() as u32;

            self.generation.push(0);

            Entity::new(id, 0)
        }
    }

    pub fn destroy(&mut self, entity: Entity) -> bool {
        if !self.is_alive(entity) {
            return false;
        }

        let generation = &mut self.generation[entity.as_usize()];

        *generation = generation
            .checked_add(1)
            .unwrap_or_else(|| panic!("{}", EntityError::GenerationOverflow));

        self.free.push(entity.index());

        true
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        let Some(generation) = self.generation.get(entity.as_usize()) else {
            return false;
        };

        *generation == entity.generation()
    }

    pub fn alive_entities(&self) -> Vec<Entity> {
        let mut alive = vec![true; self.generation.len()];

        for &index in &self.free {
            alive[index as usize] = false;
        }

        self.generation
            .iter()
            .enumerate()
            .filter(|(index, _)| alive[*index])
            .map(|(index, &generation)| Entity::new(index as u32, generation))
            .collect()
    }
}
