use crate::{QueryAccess, QueryFilter, QueryIter, QueryStorage, SystemParam, Tick, WorldCell};

use std::marker::PhantomData;

pub struct Query<'w, A: QueryAccess, F: QueryFilter = ()> {
    storage: A::Storage<'w>,
    filter: F::Storage<'w>,
    last_run: Tick,
    this_run: Tick,
    marker: PhantomData<&'w ()>,
}

impl<'w, A, F> Query<'w, A, F>
where
    A: QueryAccess,
    F: QueryFilter,
{
    pub fn new(
        storage: A::Storage<'w>,
        filter: F::Storage<'w>,
        last_run: Tick,
        this_run: Tick,
    ) -> Self {
        Self {
            storage,
            filter,
            last_run,
            this_run,
            marker: PhantomData,
        }
    }
}

impl<'w, A, F> Query<'w, A, F>
where
    A: QueryAccess,
    F: QueryFilter,
{
    pub fn len(&self) -> usize {
        let mut count = 0;

        for dense in 0..self.storage.len() {
            let entity = self.storage.entity(dense);

            if self.storage.contains(entity)
                && F::matches(&self.filter, entity, self.last_run, self.this_run)
            {
                count += 1;
            }
        }

        count
    }

    pub fn is_empty(&self) -> bool {
        (0..self.storage.len()).all(|dense| {
            let entity = self.storage.entity(dense);

            !self.storage.contains(entity)
                || !F::matches(&self.filter, entity, self.last_run, self.this_run)
        })
    }
}

impl<'w, A, F> IntoIterator for Query<'w, A, F>
where
    A: QueryAccess,
    F: QueryFilter,
{
    type Item = <A::Storage<'w> as QueryStorage<'w>>::Item;
    type IntoIter = QueryIter<'w, A::Storage<'w>, F>;

    fn into_iter(self) -> Self::IntoIter {
        QueryIter::new(self.storage, self.filter, self.last_run, self.this_run)
    }
}

impl<A, F> SystemParam for Query<'_, A, F>
where
    A: QueryAccess,
    F: QueryFilter,
{
    type Item<'w> = Query<'w, A, F>;

    fn fetch<'w>(cell: WorldCell<'w>, last_run: Tick, this_run: Tick) -> Self::Item<'w> {
        Query::new(A::fetch(cell), F::fetch(cell), last_run, this_run)
    }
}
