pub trait QueryStorage<'w>: Sized {
    type Item;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize;

    fn entity(&self, dense: usize) -> u32;

    fn contains(&self, entity: u32) -> bool;

    fn get(&mut self, dense: usize) -> Option<Self::Item>;

    fn get_by_entity(&mut self, entity: u32) -> Option<Self::Item>;
}
