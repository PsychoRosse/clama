use crate::SystemParam;

pub trait SystemParamFunction<P: SystemParam> {
    fn run<'w>(&mut self, params: P::Item<'w>);
}

impl<F: FnMut()> SystemParamFunction<()> for F {
    fn run<'w>(&mut self, _: ()) {
        self()
    }
}
