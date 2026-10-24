use crate::{Event, EventBuffer, EventStorage};

use {std::any::TypeId, std::collections::HashMap};

#[derive(Default)]
pub(crate) struct EventRegistry {
    events: HashMap<TypeId, Box<dyn EventStorage>>,
}

impl EventRegistry {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            events: HashMap::with_capacity(capacity),
        }
    }
}

impl EventRegistry {
    pub fn push<E: Event>(&mut self, event: EventBuffer<E>) {
        self.events.insert(TypeId::of::<E>(), Box::new(event));
    }

    pub fn get<E: Event>(&self) -> Option<&EventBuffer<E>> {
        self.events
            .get(&TypeId::of::<E>())
            .and_then(|event| event.as_any().downcast_ref::<EventBuffer<E>>())
    }

    pub fn get_mut<E: Event>(&mut self) -> Option<&mut EventBuffer<E>> {
        self.events
            .get_mut(&TypeId::of::<E>())
            .and_then(|event| event.as_any_mut().downcast_mut::<EventBuffer<E>>())
    }

    pub fn update(&mut self) {
        for event in self.events.values_mut() {
            event.update();
        }
    }
}
