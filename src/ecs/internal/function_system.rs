use crate::{System, SystemParam, SystemParamFunction, Tick, WorldCell};

use std::marker::PhantomData;

pub struct FunctionSystem<F, P> {
    function: F,
    last_run_tick: Tick,
    marker: PhantomData<fn() -> P>,
}

impl<F, P> FunctionSystem<F, P> {
    pub fn new(function: F) -> Self {
        Self {
            function,
            last_run_tick: Tick::ZERO,
            marker: PhantomData,
        }
    }
}

impl<F, P> System for FunctionSystem<F, P>
where
    P: SystemParam + 'static,
    F: SystemParamFunction<P> + 'static,
{
    fn run(&mut self, cell: WorldCell) {
        let this_run_tick = cell.change_tick();

        self.function
            .run(P::fetch(cell, self.last_run_tick, this_run_tick));

        self.last_run_tick = this_run_tick;
    }
}
