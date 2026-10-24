use crate::{Event, EventStorage};

use std::{any::Any, mem::swap};

pub struct EventBuffer<E: Event> {
    current: Vec<E>,
    previous: Vec<E>,
}

impl<E: Event> Default for EventBuffer<E> {
    fn default() -> Self {
        Self {
            current: Vec::new(),
            previous: Vec::new(),
        }
    }
}

impl<E: Event> EventBuffer<E> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            current: Vec::with_capacity(capacity),
            previous: Vec::with_capacity(capacity),
        }
    }
}

impl<E: Event> EventBuffer<E> {
    pub fn send(&mut self, event: E) {
        self.current.push(event);
    }

    pub fn iter(&self) -> impl Iterator<Item = &E> {
        self.previous.iter()
    }

    pub fn drain(&mut self) -> impl Iterator<Item = E> {
        self.previous.drain(..)
    }
}

impl<E: Event> EventStorage for EventBuffer<E> {
    fn update(&mut self) {
        swap(&mut self.current, &mut self.previous);
        self.current.clear();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
