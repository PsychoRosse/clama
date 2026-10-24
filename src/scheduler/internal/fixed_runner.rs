#[derive(Debug)]
pub(crate) struct FixedRunner {
    time_step: f32,
    accumulator: f32,
    max_steps: usize,
}

impl Default for FixedRunner {
    fn default() -> Self {
        Self::new(1.0 / 60.0)
    }
}

impl FixedRunner {
    pub fn new(time_step: f32) -> Self {
        Self {
            time_step,
            accumulator: 0.0,
            max_steps: 4,
        }
    }
}

impl FixedRunner {
    #[inline]
    pub fn consume(&mut self, delta: f32) -> usize {
        self.accumulator += delta;

        let mut steps = 0;

        while self.accumulator >= self.time_step && steps < self.max_steps {
            self.accumulator -= self.time_step;
            steps += 1;
        }

        steps
    }
}
