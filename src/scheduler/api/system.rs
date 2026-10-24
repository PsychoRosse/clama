use crate::WorldCell;

pub trait System: 'static {
    fn run(&mut self, cell: WorldCell);
}
