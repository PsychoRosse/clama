use crate::Registry;

pub trait Plugin: 'static {
    fn build(&self, registry: &mut impl Registry);
}
