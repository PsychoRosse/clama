#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Tick(u32);

impl Tick {
    pub const ZERO: Self = Self::new(0);
    pub const INITIAL: Self = Self::new(1);
}

impl Tick {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }
}

impl Tick {
    pub const fn get(self) -> u32 {
        self.0
    }

    pub fn increment(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }

    pub fn is_newer_than(self, last_run: Self, this_run: Self) -> bool {
        let change_age = this_run.0.wrapping_sub(self.0);
        let run_age = this_run.0.wrapping_sub(last_run.0);

        change_age < run_age && run_age < (u32::MAX / 2)
    }
}
