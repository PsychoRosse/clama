#![allow(non_snake_case)]

use crate::{QueryAccess, WorldCell};

macro_rules! impl_query_access {
    ($($A:ident),+) => {
        impl<$($A),+> QueryAccess for ($($A,)+)
        where
            $($A: QueryAccess),+
        {
            type Storage<'w> = ($($A::Storage<'w>,)+);

            fn fetch<'w>(cell: WorldCell<'w>) -> Self::Storage<'w> {
                ($($A::fetch(cell),)+)
            }
        }
    };
}

impl_query_access!(A, B);
impl_query_access!(A, B, C);
impl_query_access!(A, B, C, D);
impl_query_access!(A, B, C, D, E);
impl_query_access!(A, B, C, D, E, F);
impl_query_access!(A, B, C, D, E, F, G);
impl_query_access!(A, B, C, D, E, F, G, H);
impl_query_access!(A, B, C, D, E, F, G, H, I);
impl_query_access!(A, B, C, D, E, F, G, H, I, J);
