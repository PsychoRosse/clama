#![allow(non_snake_case)]

use crate::{SystemParam, Tick, WorldCell};

macro_rules! impl_system_param {
    ($($A:ident),+$(,)?) => {
        impl<$($A),+> SystemParam for ($($A,)+)
        where
            $($A: SystemParam),+
        {
            type Item<'w> = ($($A::Item<'w>,)+);

            fn fetch<'w>(
                cell: WorldCell<'w>,
                last_run: Tick,
                this_run: Tick,
            ) -> Self::Item<'w> {
                (
                    $(
                        $A::fetch(cell, last_run, this_run),
                    )+
                )
            }
        }
    };
}

impl_system_param!(A);
impl_system_param!(A, B);
impl_system_param!(A, B, C);
impl_system_param!(A, B, C, D);
impl_system_param!(A, B, C, D, E);
impl_system_param!(A, B, C, D, E, F);
impl_system_param!(A, B, C, D, E, F, G);
impl_system_param!(A, B, C, D, E, F, G, H);
impl_system_param!(A, B, C, D, E, F, G, H, I);
impl_system_param!(A, B, C, D, E, F, G, H, I, J);
impl_system_param!(A, B, C, D, E, F, G, H, I, J, K);
