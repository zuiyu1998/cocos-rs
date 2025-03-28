use crate::{CommandBuffer, TypeHandle};

use super::{DynPass, FrameGraph, PassNode, ResourceCreator, resource_table::ResourceTable};

#[derive(Default)]
pub struct DevicePass {
    logic_passes: Vec<LogicPass>,
}

pub struct LogicPass {
    pass: DynPass,
}

impl DevicePass {
    pub fn extra(&mut self, fg: &mut FrameGraph, handle: TypeHandle<PassNode>) {
        let pass_node = fg.get_pass_node_mut(&handle);

        let logic_pass = LogicPass {
            pass: pass_node.pass.take().unwrap(),
        };

        self.logic_passes.push(logic_pass);
    }

    pub fn execute(&mut self, creator: &impl ResourceCreator, resouce_table: &mut ResourceTable) {
        let mut command_buffer = creator.get_command_buffer();

        self.begin(&mut command_buffer);

        for logic_pass in self.logic_passes.iter_mut() {
            logic_pass.pass.execute(resouce_table);
        }

        self.end(&mut command_buffer);
    }

    pub fn begin(&self, command_buffer: &mut CommandBuffer) {
        println!("begin {:?}", command_buffer)
    }

    pub fn end(&self, command_buffer: &mut CommandBuffer) {
        println!("begin {:?}", command_buffer)
    }
}
