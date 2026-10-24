use crate::{FunctionSystem, System};

pub trait IntoSystem<Marker> {
    type System: System;

    fn into_system(self) -> Self::System;
}

impl<F> IntoSystem<fn()> for F
where
    F: FnMut() + 'static,
{
    type System = FunctionSystem<F, ()>;

    fn into_system(self) -> Self::System {
        FunctionSystem::new(self)
    }
}
