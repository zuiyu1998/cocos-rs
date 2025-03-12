use super::{
    device_pass::DevicePass,
    pass::{PassNode, PassNodeInfo},
    pass_node_builder::PassNodeBuilder,
    virtual_resources::{ResourceEntry, VirtualResource},
};
use crate::{
    RendererError,
    gfx_base::{
        Allocator, FGResource, FGResourceDescriptor, Handle, LoadOp, StoreOp, TypeEquals,
        TypedHandle,
    },
    utils::IndexHandle,
};
use std::mem::swap;

pub type StringHandle = IndexHandle<String, u32>;
pub type PassInsertPoint = u16;

pub type DynRenderFn = dyn FnOnce() -> Result<(), RendererError>;

pub struct FrameGraph {
    pub(crate) virtual_resources: Vec<Box<dyn VirtualResource>>,
    pub(crate) resource_nodes: Vec<ResourceNode>,
    pub(crate) pass_nodes: Vec<PassNode>,
    pub(crate) merge: bool,
    pub(crate) device_passes: Vec<DevicePass>,
    pub(crate) allocator: Allocator,
}

impl FrameGraph {
    pub fn new(allocator: Allocator) -> Self {
        Self {
            virtual_resources: vec![],
            resource_nodes: vec![],
            pass_nodes: vec![],
            merge: false,
            device_passes: vec![],
            allocator,
        }
    }

    pub fn execute(&mut self) {
        let mut temp: Vec<DevicePass> = vec![];

        swap(&mut temp, &mut self.device_passes);

        for mut device_pass in temp.into_iter() {
            device_pass.execute()
        }
    }

    pub fn create_pass_node_builder(
        &mut self,
        name: StringHandle,
        insert_point: PassInsertPoint,
    ) -> PassNodeBuilder {
        let pass_node = PassNode::new(insert_point, name, Handle::new(self.pass_nodes.len()));

        PassNodeBuilder {
            pass_node,
            graph: self,
        }
    }

    pub fn create_pass_node(&mut self, pass: PassNode) {
        self.pass_nodes.push(pass);
    }
    pub fn release_transient_resources(&mut self, pass_node_handle: Handle) {
        let pass_node = &mut self.pass_nodes[pass_node_handle];
        pass_node.release_transient_resources(&self.allocator, &mut self.virtual_resources);
    }

    pub fn request_transient_resources(&mut self, pass_node_handle: Handle) {
        let pass_node = &mut self.pass_nodes[pass_node_handle];
        pass_node.request_transient_resources(&self.allocator, &mut self.virtual_resources);
    }

    pub fn generate_device_passes(&mut self) {
        //todo Allocator

        let mut pass_handle = Handle::new(1);

        let mut sub_pass_node_handles: Vec<Handle> = vec![];

        let pass_node_info = self
            .pass_nodes
            .iter()
            .map(|pass_node| pass_node.to_info())
            .collect::<Vec<PassNodeInfo>>();

        for pass_node_info in pass_node_info.into_iter() {
            if pass_node_info.ref_count == 0 {
                return;
            }

            let device_pass_handle = pass_node_info.device_pass_handle;

            if pass_handle != device_pass_handle {
                let mut temp_sub_pass_node_handles = vec![];
                swap(&mut temp_sub_pass_node_handles, &mut sub_pass_node_handles);

                for pass_node_handle in temp_sub_pass_node_handles.iter() {
                    self.release_transient_resources(*pass_node_handle);
                }

                let device_pass = DevicePass::new(self, temp_sub_pass_node_handles);

                self.device_passes.push(device_pass);

                pass_handle = device_pass_handle;
            } else {
                self.request_transient_resources(pass_node_info.handle);
            }
        }
    }

    pub fn compute_store_action_and_memory_less(&mut self) {
        let mut pass_handle = Handle::new(0);
        let mut last_pass_subpass_enable = false;

        //更新pass_node的device_pass_id
        for pass_node in self.pass_nodes.iter_mut() {
            if pass_node.ref_count == 0 {
                continue;
            }

            let old_pass_handle = pass_handle;

            let update: usize =
                if !pass_node.subpass || last_pass_subpass_enable != pass_node.subpass {
                    1
                } else {
                    0
                };
            pass_handle = pass_handle + update;

            let update = if old_pass_handle == pass_handle {
                pass_node.has_cleared_attachment as usize
                    * !pass_node.clear_action_ignorable as usize
            } else {
                0
            };

            pass_handle = pass_handle + update;

            pass_node.device_pass_handle = pass_handle;

            last_pass_subpass_enable = pass_node.subpass && !pass_node.subpass_end;
        }

        let mut resource_ids = vec![];

        for pass_node_index in 0..self.pass_nodes.len() {
            let pass_node_info = self.pass_nodes[pass_node_index].to_info();
            if pass_node_info.ref_count == 0 {
                continue;
            }

            for (attachment_index, attachment_info) in
                pass_node_info.attachments_infos.iter().enumerate()
            {
                let resource_node_info = self
                    .get_resource_node(attachment_info.texture_handle)
                    .to_info();

                let info =
                    self.virtual_resources[resource_node_info.virtual_resource_handle].info();

                let last_pass_node_device_pass_handle =
                    self.pass_nodes[info.last_pass_index.unwrap()].device_pass_handle;

                if info.imported || resource_node_info.reader_count == 0 {
                    if pass_node_info.subpass {
                        if pass_node_info.device_pass_handle != last_pass_node_device_pass_handle {
                            self.pass_nodes[pass_node_index].attachments[attachment_index]
                                .store_op = StoreOp::Store;
                        }
                    } else if attachment_info.write_mask != 0 {
                        self.pass_nodes[pass_node_index].attachments[attachment_index].store_op =
                            StoreOp::Store;
                    }
                }

                if pass_node_info.subpass
                    && attachment_info.load_op == LoadOp::Load
                    && resource_node_info.version > 1
                {
                    if let Some(new_version_resource_node_info) = self
                        .get_resource_node_with_version(
                            resource_node_info.virtual_resource_handle,
                            resource_node_info.version - 1,
                        )
                        .map(|node| node.to_info())
                    {
                        let write_pass_node_info = self.pass_nodes[new_version_resource_node_info
                            .pass_node_writer_handle
                            .unwrap()]
                        .to_info();

                        if write_pass_node_info.device_pass_handle
                            == pass_node_info.device_pass_handle
                        {
                            self.pass_nodes[pass_node_index].attachments[attachment_index]
                                .store_op = StoreOp::Store;

                            if let Some(writer_attachment_index) = self.pass_nodes
                                [new_version_resource_node_info
                                    .pass_node_writer_handle
                                    .unwrap()]
                            .get_render_target_attachment_index(
                                self,
                                new_version_resource_node_info.virtual_resource_handle,
                            ) {
                                self.pass_nodes[new_version_resource_node_info
                                    .pass_node_writer_handle
                                    .unwrap()]
                                .attachments[writer_attachment_index]
                                    .store_op = StoreOp::Discard
                            }
                        }
                    }
                }

                if attachment_info.load_op == LoadOp::Load {
                    let info = self.virtual_resources[resource_node_info.virtual_resource_handle]
                        .info_mut();

                    info.never_loaded = false;
                }

                if attachment_info.store_op == StoreOp::Store {
                    let info = self.virtual_resources[resource_node_info.virtual_resource_handle]
                        .info_mut();

                    info.never_stored = false;
                }

                resource_ids.push(resource_node_info.virtual_resource_handle);
            }
        }

        //todo update memoryless and memorylessMSAA
        // for resource_id in resource_ids.into_iter() {}
    }

    pub fn get_resource_node(&self, handle: Handle) -> &ResourceNode {
        &self.resource_nodes[handle]
    }

    pub fn get_resource_node_with_version(
        &self,
        handle: Handle,
        version: u8,
    ) -> Option<&ResourceNode> {
        if self.resource_nodes[handle].version == version {
            Some(&self.resource_nodes[handle])
        } else {
            None
        }
    }

    pub fn compute_resource_lifetime(&mut self) {
        for pass_node in self.pass_nodes.iter_mut() {
            if pass_node.ref_count == 0 {
                continue;
            }

            //更新渲染节点读取的资源节点所指向资源的生命周期
            for resource_index in pass_node.reads.iter() {
                let resource_node = &self.resource_nodes[*resource_index];
                let resource = &mut self.virtual_resources[resource_node.virtual_resource_handle];
                resource.info_mut().update_lifetime(pass_node);
            }

            //更新渲染节点吸入的资源节点所指向资源的生命周期
            for resource_index in pass_node.writes.iter() {
                let resource_node = &self.resource_nodes[*resource_index];
                let resource = &mut self.virtual_resources[resource_node.virtual_resource_handle];
                let info = resource.info_mut();
                info.update_lifetime(pass_node);
                info.writer_count += 1;
            }

            pass_node.attachments.sort();
        }

        //更新pass_node中资源使用的索引顺序
        for resource_index in 0..self.virtual_resources.len() {
            let resource = &self.virtual_resources[resource_index];
            let info = resource.info();
            if info.first_pass_index.is_none() || info.last_pass_index.is_none() {
                continue;
            }

            let last_pass_index = info.last_pass_index.unwrap();
            let pass_node = &self.pass_nodes[last_pass_index];
            let has_attachment = pass_node
                .get_render_target_attachment(self, info.handle)
                .is_some();

            if info.ref_count == 0 && !has_attachment {
                continue;
            }

            let first_pass_index = info.first_pass_index.unwrap();

            let first_pass_node = &mut self.pass_nodes[first_pass_index];
            first_pass_node.resource_request_array.push(info.handle);

            let last_pass_node = &mut self.pass_nodes[last_pass_index];
            last_pass_node.resource_release_array.push(info.handle);
        }
    }

    pub fn sort(&mut self) {
        self.pass_nodes
            .sort_by(|a, b| a.insert_point.cmp(&b.insert_point));
    }

    pub fn cull(&mut self) {
        //更新pass_node的引用
        for pass_node in self.pass_nodes.iter_mut() {
            pass_node.ref_count = pass_node.writes.len() as u32;
            if pass_node.side_effect {
                pass_node.ref_count += 1;
            }

            for resource_index in pass_node.reads.iter() {
                let resource_node = &mut self.resource_nodes[*resource_index];
                resource_node.reader_count += 1;
            }
        }

        let mut resource_handle_stack: Vec<Handle> = vec![];

        //记录所有要被剔除的资源节点
        for resource_node_info in self.resource_nodes.iter().map(|node| node.to_info()) {
            if resource_node_info.reader_count == 0
                && resource_node_info.pass_node_writer_handle.is_some()
            {
                resource_handle_stack.push(resource_node_info.handle);
            }
        }

        //删除资源节点引用的pass_node计数
        while !resource_handle_stack.is_empty() {
            let resource_node = &self.resource_nodes[resource_handle_stack.pop().unwrap()];

            let pass_node_writer =
                &mut self.pass_nodes[resource_node.pass_node_writer_handle.unwrap()];

            pass_node_writer.ref_count -= 1;

            //去除pass_node读取的资源节点引用
            if pass_node_writer.ref_count == 0 {
                for resource_index in pass_node_writer.reads.iter() {
                    let resource_node = &mut self.resource_nodes[*resource_index];
                    resource_node.reader_count -= 1;

                    if resource_node.reader_count == 0 {
                        resource_handle_stack.push(*resource_index);
                    }
                }
            }
        }

        //更新资源节点对应的虚拟资源引用
        for resource_node in self.resource_nodes.iter() {
            let resource = &mut self.virtual_resources[resource_node.virtual_resource_handle];
            resource.info_mut().ref_count += 1;
        }
    }

    pub fn merge_pass_nodes(&mut self) {
        let count = Handle::new(self.pass_nodes.len());
        let mut current_pass_id = Handle::new(0);
        let mut last_pass_id;

        //获取最近且有效的pass node
        while current_pass_id < count {
            let pass_node = &self.pass_nodes[current_pass_id];

            if pass_node.ref_count != 0 {
                break;
            }
            current_pass_id = current_pass_id + 1;
        }

        last_pass_id = current_pass_id;

        while {
            current_pass_id = current_pass_id + 1;
            current_pass_id < count
        } {
            let current_pass_node = &self.pass_nodes[current_pass_id];
            //寻找下一个有效的pass_node
            if current_pass_node.ref_count == 0 {
                continue;
            }

            let last_pass_node = &self.pass_nodes[last_pass_id];

            let merge = last_pass_node.can_merge(self, current_pass_node);

            if !merge {
                last_pass_id = current_pass_id;
            } else {
                let mut distance = 1;

                let prev_pass_node_id = {
                    let mut prev_pass_node: &PassNode = &self.pass_nodes[last_pass_id];
                    //寻找last_pass_node到current_pass_node中间断裂的pass_node
                    while prev_pass_node.next_pass_node_handle.is_some() {
                        prev_pass_node =
                            &self.pass_nodes[prev_pass_node.next_pass_node_handle.unwrap()];

                        distance += 1;
                    }

                    prev_pass_node.handle
                };

                let prev_pass_node = &mut self.pass_nodes[prev_pass_node_id];
                prev_pass_node.next_pass_node_handle = Some(current_pass_id);

                let current_pass_node = &mut self.pass_nodes[current_pass_id];

                current_pass_node.next_pass_node_handle = Some(last_pass_id);
                current_pass_node.distance_to_headad = distance;
                current_pass_node.ref_count = 0;

                let last_pass_node = &self.pass_nodes[last_pass_id];
                let current_pass_node = &self.pass_nodes[current_pass_id];

                let attachment_count = last_pass_node.attachments.len();

                for i in 0..attachment_count {
                    let attachment_in_last_pass_node = &last_pass_node.attachments[i];
                    let attachment_in_current_pass_node = &current_pass_node.attachments[i];

                    let reader_count = self.resource_nodes
                        [attachment_in_current_pass_node.to_info().texture_handle]
                        .reader_count;

                    let resource_node = &mut self.resource_nodes
                        [attachment_in_last_pass_node.to_info().texture_handle];

                    let write_count = self.virtual_resources[resource_node.virtual_resource_handle]
                        .info()
                        .writer_count;

                    assert_eq!(write_count, 1);

                    self.virtual_resources[resource_node.virtual_resource_handle]
                        .info_mut()
                        .writer_count -= 1;

                    resource_node.reader_count += reader_count;

                    let reader_count =
                        if attachment_in_current_pass_node.desc.load_op == LoadOp::Load {
                            1
                        } else {
                            0
                        };

                    resource_node.reader_count -= reader_count;
                }
            }
        }
    }

    pub fn compile(&mut self) {
        if self.pass_nodes.is_empty() {
            return;
        }

        self.sort();
        self.cull();
        self.compute_resource_lifetime();

        if self.merge {
            self.merge_pass_nodes();
        }

        self.compute_store_action_and_memory_less();
        self.generate_device_passes();
    }

    pub fn create<DescriptorType>(&mut self, name: StringHandle, desc: DescriptorType) -> TypedHandle<DescriptorType::Resource>
    where
        DescriptorType: FGResourceDescriptor + TypeEquals<Other = <<DescriptorType as FGResourceDescriptor>::Resource as FGResource>::Descriptor>,
    {
        let virtual_resource: Box<dyn VirtualResource> =
            Box::new(ResourceEntry::<DescriptorType::Resource>::new(
                Handle::new(self.virtual_resources.len()),
                name,
                TypeEquals::same(desc),
            ));

        let index = self.create_resource_node(virtual_resource);

        TypedHandle::new(index)
    }

    ///指向已存在的资源
    pub fn create_resource_node_with_id(&mut self, virtual_resource_handle: Handle) -> Handle {
        let version = self.virtual_resources[virtual_resource_handle]
            .info()
            .version;
        let handle = Handle::new(self.resource_nodes.len());

        self.resource_nodes
            .push(ResourceNode::new(handle, virtual_resource_handle, version));

        handle
    }

    pub(crate) fn create_resource_node(
        &mut self,
        virtual_resource: Box<dyn VirtualResource>,
    ) -> Handle {
        let virtual_resource_handle = virtual_resource.info().handle;
        let version = virtual_resource.info().version;
        self.virtual_resources.push(virtual_resource);

        let handle = Handle::new(self.resource_nodes.len());

        self.resource_nodes
            .push(ResourceNode::new(handle, virtual_resource_handle, version));

        handle
    }
}

pub struct ResourceNode {
    pub virtual_resource_handle: Handle,
    version: u8,
    reader_count: u32,
    pub pass_node_writer_handle: Option<Handle>,
    pub handle: Handle,
}

pub struct ResourceNodeInfo {
    pub virtual_resource_handle: Handle,
    pub version: u8,
    pub reader_count: u32,
    pub pass_node_writer_handle: Option<Handle>,
    pub handle: Handle,
}

impl ResourceNode {
    pub fn to_info(&self) -> ResourceNodeInfo {
        ResourceNodeInfo {
            virtual_resource_handle: self.virtual_resource_handle,
            version: self.version,
            reader_count: self.reader_count,
            pass_node_writer_handle: self.pass_node_writer_handle,
            handle: self.handle,
        }
    }

    pub fn new(handle: Handle, virtual_resource_handle: Handle, version: u8) -> Self {
        Self {
            virtual_resource_handle,
            version,
            reader_count: 0,
            pass_node_writer_handle: None,
            handle,
        }
    }
}
