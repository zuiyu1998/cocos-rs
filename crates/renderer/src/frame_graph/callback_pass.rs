use super::{Pass, PassNodeBuilder, resource_table::ResourceTable};

pub type SetupFn<Data> = Box<dyn FnOnce(&mut PassNodeBuilder, &mut Data)>;
pub type ExecuteFn<Data> = Box<dyn FnOnce(&Data, &mut ResourceTable)>;

pub struct CallbackPass<Data> {
    data: Data,
    setup: Option<SetupFn<Data>>,
    execute: Option<ExecuteFn<Data>>,
}

impl<Data> CallbackPass<Data>
where
    Data: Default,
{
    pub fn new(
        setup: impl FnOnce(&mut PassNodeBuilder, &mut Data) + 'static,
        execute: impl FnOnce(&Data, &mut ResourceTable) + 'static,
    ) -> Self {
        CallbackPass {
            data: Data::default(),
            setup: Some(Box::new(setup)),
            execute: Some(Box::new(execute)),
        }
    }
}

impl<Data> Pass for CallbackPass<Data>
where
    Data: Default,
{
    fn setup(&mut self, builder: &mut PassNodeBuilder) {
        if let Some(setup) = self.setup.take() {
            setup(builder, &mut self.data);
        }
    }

    fn execute(&mut self, resouce_table: &mut ResourceTable) {
        if let Some(execute) = self.execute.take() {
            execute(&self.data, resouce_table);
        }
    }
}
