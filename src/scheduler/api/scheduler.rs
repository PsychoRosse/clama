use crate::{FixedRunner, IntoSystem, Schedule, Stage, WorldCell};

use std::array::from_fn;

pub struct Scheduler {
    schedules: [Schedule; Stage::COUNT],
    fixed_runner: FixedRunner,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self {
            schedules: from_fn(|_| Schedule::default()),
            fixed_runner: FixedRunner::default(),
        }
    }
}

impl Scheduler {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            schedules: from_fn(|_| Schedule::with_capacity(capacity)),
            fixed_runner: FixedRunner::default(),
        }
    }
}

impl Scheduler {
    #[inline]
    pub fn run_full(&mut self, cell: WorldCell, delta: f32) {
        let fixed_steps = self.fixed_runner.consume(delta);

        for _ in 0..fixed_steps {
            self.run(Stage::PreFixedUpdate, cell);
            self.run(Stage::FixedUpdate, cell);
            self.run(Stage::PostFixedUpdate, cell);
        }

        self.run(Stage::TimeUpdate, cell);
        self.run(Stage::PreInputUpdate, cell);

        self.run(Stage::PreUpdate, cell);
        self.run(Stage::Update, cell);
        self.run(Stage::LateUpdate, cell);

        self.run(Stage::PreRender, cell);
        self.run(Stage::Render, cell);
        self.run(Stage::PostRender, cell);

        self.run(Stage::PostInputUpdate, cell);
    }

    #[inline]
    fn run(&mut self, stage: Stage, cell: WorldCell) {
        self.schedules[stage.as_usize()].run(cell);
    }
}

impl Scheduler {
    pub fn add<M, S>(&mut self, stage: Stage, system: S)
    where
        S: IntoSystem<M>,
    {
        self.schedules[stage.as_usize()].add(system.into_system());
    }

    pub fn run_startup(&mut self, cell: WorldCell) {
        self.run(Stage::Startup, cell);
    }
}
