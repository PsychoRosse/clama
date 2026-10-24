#![allow(non_snake_case)]

use crate::{FunctionSystem, IntoSystem, SystemParam};

macro_rules! impl_into_system {
    ($($A:ident),+) => {
        impl<F, $($A),+> IntoSystem<fn($($A),+)> for F
        where
            $($A: SystemParam + 'static),+,
            F: FnMut($($A),+) + for<'w> FnMut($($A::Item<'w>),+) + 'static,
        {
            type System = FunctionSystem<F, ($($A,)+)>;

            fn into_system(self) -> Self::System {
                FunctionSystem::new(self)
            }
        }
    };
}

impl_into_system!(A);
impl_into_system!(A, B);
impl_into_system!(A, B, C);
impl_into_system!(A, B, C, D);
impl_into_system!(A, B, C, D, E);
impl_into_system!(A, B, C, D, E, G);
impl_into_system!(A, B, C, D, E, G, H);
impl_into_system!(A, B, C, D, E, G, H, I);
impl_into_system!(A, B, C, D, E, G, H, I, J);
impl_into_system!(A, B, C, D, E, G, H, I, J, K);
