#![allow(non_snake_case)]

use crate::{SystemParam, SystemParamFunction};

macro_rules! impl_system_param_function {
    ($($A:ident),+) => {
        impl<F, $($A),+> SystemParamFunction<($($A,)+)> for F
        where
            $($A: SystemParam),+,
            F: FnMut($($A),+) + for<'w> FnMut($($A::Item<'w>),+)
        {
            fn run<'w>(&mut self, params: ($($A::Item<'w>,)+),) {
                let ($($A,)+) = params;
                self ($($A,)+)
            }
        }
    };
}

impl_system_param_function!(A);
impl_system_param_function!(A, B);
impl_system_param_function!(A, B, C);
impl_system_param_function!(A, B, C, D);
impl_system_param_function!(A, B, C, D, E);
impl_system_param_function!(A, B, C, D, E, G);
impl_system_param_function!(A, B, C, D, E, G, H);
impl_system_param_function!(A, B, C, D, E, G, H, I);
impl_system_param_function!(A, B, C, D, E, G, H, I, J);
impl_system_param_function!(A, B, C, D, E, G, H, I, J, K);
