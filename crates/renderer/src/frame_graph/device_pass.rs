use std::mem::swap;

use crate::{RenderPassInfo, TypeHandle};

use super::{
    DynPass, FrameGraph, PassNode, RenderContext, ResourceTable, TransientResourceCache,
    VirtualResource,
};

pub struct DevicePass {
    logic_passes: Vec<LogicPass>,
    resource_table: ResourceTable,
}

pub struct LogicPass {
    pass: DynPass,
    resource_release_array: Vec<TypeHandle<VirtualResource>>,
}

impl LogicPass {
    pub fn release_resources(
        &mut self,
        resource_table: &mut ResourceTable,
        transient_resource_cache: &mut TransientResourceCache,
    ) {
        for handle in self.resource_release_array.iter() {
            resource_table.release_resource(handle, transient_resource_cache);
        }
    }
}

impl DevicePass {
    pub fn new(resource_table: ResourceTable) -> DevicePass {
        Self {
            logic_passes: vec![],
            resource_table,
        }
    }

    pub fn extra(&mut self, fg: &mut FrameGraph, handle: TypeHandle<PassNode>) {
        let pass_node = fg.get_pass_node_mut(&handle);

        let logic_pass = LogicPass {
            pass: pass_node.pass.take().unwrap(),
            resource_release_array: pass_node.resource_release_array.clone(),
        };

        self.logic_passes.push(logic_pass);
    }

    pub fn execute(&mut self, render_context: &mut RenderContext) {
        self.begin(render_context);

        for logic_pass in self.logic_passes.iter_mut() {
            logic_pass.pass.execute(render_context);

            logic_pass.release_resources(&mut render_context.resource_table, render_context.transient_resource_cache);
        }

        self.end(render_context);
    }

    pub fn begin(&mut self, render_context: &mut RenderContext) {
        swap(
            &mut self.resource_table,
            &mut render_context.resource_table,
        );

        let mut command_buffer = render_context.device().create_command_buffer();

        let render_pass_info: RenderPassInfo = RenderPassInfo::new();
        command_buffer.begin_render_pass(render_pass_info);

        render_context.set_cb(command_buffer);
    }

    pub fn end(&self, render_context: &mut RenderContext) {
        render_context.resource_table = ResourceTable::default();

        if let Some(mut command_buffer) = render_context.take_cb() {
            command_buffer.end_render_pass();
        }
    }
}
