use crate::{System, WorldCell};

#[derive(Default)]
pub(crate) struct Schedule {
    systems: Vec<Box<dyn System>>,
}

impl Schedule {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            systems: Vec::with_capacity(capacity),
        }
    }
}

impl Schedule {
    #[inline]
    pub fn run(&mut self, world: WorldCell) {
        for system in &mut self.systems {
            system.run(world);
            world.advance_change_tick();
        }
    }
}

impl Schedule {
    pub fn add<S: System>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }
}
