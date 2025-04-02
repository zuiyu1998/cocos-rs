use crate::{Device, gfx_base::TypeHandle};

use super::{
    CallbackPass, DevicePass, DynPass, FGResource, FGResourceDescriptor, PassNode, PassNodeBuilder,
    RenderContext, ResourceInfo, ResourceNode, ResourceNodeHandle, ResourceTable,
    TransientResourceCache, TypeEquals, VirtualResource,
};

#[derive(Default)]
pub struct FrameGraph {
    pass_nodes: Vec<PassNode>,
    resource_nodes: Vec<ResourceNode>,
    resources: Vec<VirtualResource>,
    device_passes: Option<Vec<DevicePass>>,
}

impl FrameGraph {
    pub fn reset(&mut self) {
        *self = FrameGraph::default();
    }

    pub fn execute(
        &mut self,
        device: &Device,
        transient_resource_cache: &mut TransientResourceCache,
    ) {
        let mut render_context = RenderContext::new(device, transient_resource_cache);

        let device_passes = self.device_passes.take().unwrap();

        for mut device_pass in device_passes {
            device_pass.execute(&mut render_context);
        }
    }

    pub fn compile(
        &mut self,
        device: &Device,
        transient_resource_cache: &mut TransientResourceCache,
    ) {
        if self.pass_nodes.is_empty() {
            return;
        }

        self.sort();
        //todo cull

        self.compute_resource_lifetime();

        // self.compiled_pipelines(pipeline_cache);

        self.generate_device_passes(device, transient_resource_cache);
    }

    fn generate_device_passes(
        &mut self,
        device: &Device,
        transient_resource_cache: &mut TransientResourceCache,
    ) {
        let mut device_passes = vec![];

        for index in 0..self.pass_nodes.len() {
            let pass_node_handle = self.pass_nodes[index].handle;

            let mut resource_table: ResourceTable = ResourceTable::default();

            for resource_handle in self
                .get_pass_node(&pass_node_handle)
                .resource_request_array
                .clone()
            {
                let resource = self.get_resource(&resource_handle);
                resource_table.request_resources(resource, device, transient_resource_cache);
            }

            let mut device_pass = DevicePass::new(resource_table);

            device_pass.extra(self, pass_node_handle);

            device_passes.push(device_pass);
        }

        self.device_passes = Some(device_passes);
    }

    fn sort(&mut self) {
        self.pass_nodes
            .sort_by(|a, b| a.insert_point.cmp(&b.insert_point));
    }

    fn compute_resource_lifetime(&mut self) {
        for pass_node in self.pass_nodes.iter_mut() {
            //更新渲染节点读取的资源节点所指向资源的生命周期
            for resource_node_handle in pass_node.reads.iter() {
                let resource_node = &self.resource_nodes[resource_node_handle.index()];
                let resource = &mut self.resources[resource_node.resource_handle.index()];
                resource.info.update_lifetime(pass_node.handle);
            }

            //更新渲染节点吸入的资源节点所指向资源的生命周期
            for resource_node_handle in pass_node.writes.iter() {
                let resource_node = &self.resource_nodes[resource_node_handle.index()];
                let resource = &mut self.resources[resource_node.resource_handle.index()];
                resource.info.update_lifetime(pass_node.handle);
            }
        }

        //更新pass_node中资源使用的索引顺序
        for resource_index in 0..self.resources.len() {
            let resource = &self.resources[resource_index];
            let info = resource.info.clone();

            if info.first_pass_node_handle.is_none() || info.last_pass_node_handle.is_none() {
                continue;
            }

            let first_pass_node_handle = info.first_pass_node_handle.unwrap();
            let first_pass_node = &mut self.pass_nodes[first_pass_node_handle.index()];
            first_pass_node.resource_request_array.push(info.handle);

            let last_pass_node_handle = info.last_pass_node_handle.unwrap();
            let last_pass_node = &mut self.pass_nodes[last_pass_node_handle.index()];
            last_pass_node.resource_release_array.push(info.handle);
        }
    }
}

impl FrameGraph {
    pub fn create<DescriptorType>(&mut self, name: &str, desc: DescriptorType) -> ResourceNodeHandle<DescriptorType::Resource>
    where
        DescriptorType: FGResourceDescriptor + TypeEquals<Other = <<DescriptorType as FGResourceDescriptor>::Resource as FGResource>::Descriptor>,
    {
        let resource_handle = TypeHandle::new(self.resources.len());

        let resource: VirtualResource = VirtualResource::setup::<DescriptorType::Resource>(
            name,
            resource_handle,
            TypeEquals::same(desc),
        );

        let resource_info = resource.info.clone();
        self.resources.push(resource);

        let handle = self.create_resource_node(resource_info);

        ResourceNodeHandle::new(handle, resource_handle)
    }

    pub fn create_resource_node(
        &mut self,
        resource_info: ResourceInfo,
    ) -> TypeHandle<ResourceNode> {
        let resource_handle = resource_info.handle;
        let version = resource_info.version();

        let handle = TypeHandle::new(self.resource_nodes.len());

        self.resource_nodes
            .push(ResourceNode::new(handle, resource_handle, version));

        handle
    }

    pub fn add_callback_pass<Data, Setup, Execute>(
        &mut self,
        insert_point: usize,
        name: &str,
        setup: Setup,
        execute: Execute,
    ) where
        Data: Default + 'static,
        Setup: FnOnce(&mut PassNodeBuilder, &mut Data) + 'static,
        Execute: FnOnce(&Data, &mut RenderContext) + 'static,
    {
        let pass = CallbackPass::new(setup, execute);

        self.add_pass(insert_point, name, Box::new(pass));
    }

    pub fn add_pass(&mut self, insert_point: usize, name: &str, mut pass: DynPass) {
        let handle = self.get_current_pass_node_handle();
        let mut builder = PassNodeBuilder::new(insert_point, name, handle, self);

        pass.setup(&mut builder);

        let mut pass_node = builder.build();
        pass_node.pass = Some(pass);

        self.pass_nodes.push(pass_node);
    }

    pub fn get_current_pass_node_handle(&self) -> TypeHandle<PassNode> {
        TypeHandle::new(self.pass_nodes.len())
    }

    pub fn get_pass_node_mut(&mut self, handle: &TypeHandle<PassNode>) -> &mut PassNode {
        &mut self.pass_nodes[handle.index()]
    }

    pub fn get_pass_node(&self, handle: &TypeHandle<PassNode>) -> &PassNode {
        &self.pass_nodes[handle.index()]
    }

    pub fn get_resource_node(&self, handle: &TypeHandle<ResourceNode>) -> &ResourceNode {
        &self.resource_nodes[handle.index()]
    }

    pub fn get_resource_node_mut(
        &mut self,
        handle: &TypeHandle<ResourceNode>,
    ) -> &mut ResourceNode {
        &mut self.resource_nodes[handle.index()]
    }

    pub fn get_resource(&self, handle: &TypeHandle<VirtualResource>) -> &VirtualResource {
        &self.resources[handle.index()]
    }

    pub fn get_resource_mut(
        &mut self,
        handle: &TypeHandle<VirtualResource>,
    ) -> &mut VirtualResource {
        &mut self.resources[handle.index()]
    }
}
