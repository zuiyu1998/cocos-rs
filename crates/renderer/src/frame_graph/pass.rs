use super::{PassNodeBuilder, resource_table::ResourceTable};

pub trait Pass {
    fn setup(&mut self, builder: &mut PassNodeBuilder);

    fn execute(&mut self, resouce_table: &mut ResourceTable);
}

pub type DynPass = Box<dyn Pass>;
