#![allow(non_snake_case)]

use crate::{QueryFilter, Tick, WorldCell};

macro_rules! impl_query_filter {
    ($($A:ident : $idx:tt),+$(,)?) => {
        impl<$($A),+> QueryFilter for ($($A,)+)
        where
            $($A: QueryFilter),+
        {
            type Storage<'w> = ($($A::Storage<'w>,)+);

            fn fetch<'w>(cell: WorldCell<'w>) -> Self::Storage<'w> {
                ($($A::fetch(cell),)+)
            }

            fn matches(
                storage: &Self::Storage<'_>,
                entity: u32,
                last_run: Tick,
                this_run: Tick,
            ) -> bool {
                true $(&& $A::matches(
                    &storage.$idx,
                    entity,
                    last_run,
                    this_run,
                ))+
            }
        }
    };
}

impl_query_filter!(A: 0, B: 1);
impl_query_filter!(A: 0, B: 1, C: 2);
impl_query_filter!(A: 0, B: 1, C: 2, D: 3);
impl_query_filter!(A: 0, B: 1, C: 2, D: 3, E: 4);
impl_query_filter!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5);
impl_query_filter!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6);
impl_query_filter!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7);
impl_query_filter!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8);
impl_query_filter!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9);
