use crate::{EntityStorage, QueryAccess, WorldCell};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Entity {
    index: u32,
    generation: u32,
}

impl Entity {
    pub fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }
}

impl Entity {
    pub fn index(self) -> u32 {
        self.index
    }

    pub fn generation(self) -> u32 {
        self.generation
    }

    pub fn as_usize(self) -> usize {
        self.index as usize
    }
}

impl QueryAccess for Entity {
    type Storage<'w> = EntityStorage;

    fn fetch<'w>(world: WorldCell<'w>) -> Self::Storage<'w> {
        EntityStorage::new(world.allocator())
    }
}
