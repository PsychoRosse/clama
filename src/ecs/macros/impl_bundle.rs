#![allow(non_snake_case)]

use crate::{Bundle, Component, Entity, WorldCell};

macro_rules! impl_bundle {
    ($($C:ident),+) => {
        impl<$($C),+> Bundle for ($($C,)+)
        where
            $($C: Component),+
        {
            fn insert(self, entity: Entity, cell: WorldCell) {
                let ($($C,)+) = self;

                $(
                    cell.with_component(entity, $C);
                )+
            }
        }
    };
}

impl_bundle!(A);
impl_bundle!(A, B);
impl_bundle!(A, B, C);
impl_bundle!(A, B, C, D);
impl_bundle!(A, B, C, D, E);
impl_bundle!(A, B, C, D, E, F);
impl_bundle!(A, B, C, D, E, F, G);
impl_bundle!(A, B, C, D, E, F, G, H);
impl_bundle!(A, B, C, D, E, F, G, H, I);
impl_bundle!(A, B, C, D, E, F, G, H, I, J);
