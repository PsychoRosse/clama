use crate::{Event, EventBuffer, SystemParam, Tick, WorldCell};

pub struct EventReader<'w, E: Event> {
    value: &'w EventBuffer<E>,
}

impl<'w, E: Event> EventReader<'w, E> {
    pub fn new(value: &'w EventBuffer<E>) -> Self {
        Self { value }
    }
}

impl<'w, E: Event> EventReader<'w, E> {
    pub fn iter(&self) -> impl Iterator<Item = &E> {
        self.value.iter()
    }

    pub fn get(&self) -> &EventBuffer<E> {
        self.value
    }
}

impl<E: Event> SystemParam for EventReader<'_, E> {
    type Item<'w> = EventReader<'w, E>;

    fn fetch<'w>(cell: WorldCell<'w>, _: Tick, _: Tick) -> Self::Item<'w> {
        EventReader::new(cell.get_event_now::<E>())
    }
}
