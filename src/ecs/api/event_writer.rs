use crate::{Event, EventBuffer, SystemParam, Tick, WorldCell};

pub struct EventWriter<'w, E: Event> {
    value: &'w mut EventBuffer<E>,
}

impl<'w, E: Event> EventWriter<'w, E> {
    pub fn new(value: &'w mut EventBuffer<E>) -> Self {
        Self { value }
    }
}

impl<'w, E: Event> EventWriter<'w, E> {
    pub fn send(&mut self, event: E) {
        self.value.send(event);
    }

    pub fn get(&mut self) -> &mut EventBuffer<E> {
        self.value
    }
}

impl<E: Event> SystemParam for EventWriter<'_, E> {
    type Item<'w> = EventWriter<'w, E>;

    fn fetch<'w>(cell: WorldCell<'w>, _: Tick, _: Tick) -> Self::Item<'w> {
        EventWriter::new(cell.get_event_mut_now::<E>())
    }
}
