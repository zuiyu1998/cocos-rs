use super::{
    FrameResource, FrameResourceDescriptor, TypeEquals,
    handle::TypedHandle,
    pass::PassNode,
    pass_node_builder::PassNodeBuilder,
    virtual_resources::{ResourceEntry, VirtualResource},
};
use crate::{
    RendererError,
    gfx_base::{LoadOp, StoreOp},
    utils::IndexHandle,
};
use std::mem::swap;

pub type StringHandle = IndexHandle<String, u32>;
pub type PassInsertPoint = u16;

#[derive(Default)]
pub struct FrameGraph {
    pub(crate) virtual_resources: Vec<Box<dyn VirtualResource>>,
    pub(crate) resource_nodes: Vec<ResourceNode>,
    pub(crate) pass_nodes: Vec<PassNode>,
    pub(crate) merge: bool,
    pub(crate) device_passes: Vec<DevicePass>,
}

pub struct DevicePass {}

impl DevicePass {
    pub fn new(_garph: &FrameGraph, _pass_nodes: Vec<PassNode>) -> Self {
        DevicePass {}
    }
}

impl FrameGraph {
    pub fn create_pass_node_builder(
        &mut self,
        name: StringHandle,
        insert_point: PassInsertPoint,
    ) -> PassNodeBuilder {
        let pass_node = PassNode::new(insert_point, name, self.pass_nodes.len());

        PassNodeBuilder {
            pass_node,
            graph: self,
        }
    }

    pub fn create_pass_node(&mut self, pass: PassNode) {
        self.pass_nodes.push(pass);
    }

    pub fn generate_device_passes(&mut self) {
        //todo Allocator

        let mut pass_id = 1;

        let mut temp: Vec<PassNode> = vec![];
        swap(&mut temp, &mut self.pass_nodes);

        let mut sub_pass_nodes: Vec<PassNode> = vec![];

        for mut pass_node in temp.into_iter() {
            if pass_node.ref_count == 0 {
                return;
            }

            let device_pass_id = pass_node.device_pass_id;

            if pass_id != device_pass_id {
                let mut temp_sub_pass_nodes = vec![];
                swap(&mut temp_sub_pass_nodes, &mut sub_pass_nodes);

                for pass_node in temp_sub_pass_nodes.iter_mut() {
                    pass_node.release_transient_resources(&mut self.virtual_resources);
                }

                let device_pass = DevicePass::new(self, temp_sub_pass_nodes);

                self.device_passes.push(device_pass);

                pass_id = device_pass_id;
            } else {
                pass_node.request_transient_resources(&mut self.virtual_resources);
                sub_pass_nodes.push(pass_node);
            }
        }
    }

    pub fn compute_store_action_and_memory_less(&mut self) {
        let mut pass_id = 0;
        let mut last_pass_subpass_enable = false;

        //更新pass_node的device_pass_id
        for pass_node in self.pass_nodes.iter_mut() {
            if pass_node.ref_count == 0 {
                continue;
            }

            let old_pass_id = pass_id;

            pass_id += if !pass_node.subpass || last_pass_subpass_enable != pass_node.subpass {
                1
            } else {
                0
            };

            pass_id += if old_pass_id == pass_id {
                pass_node.has_cleared_attachment as usize
                    * !pass_node.clear_action_ignorable as usize
            } else {
                0
            };

            pass_node.device_pass_id = pass_id;

            last_pass_subpass_enable = pass_node.subpass && !pass_node.subpass_end;
        }

        let mut resource_ids = vec![];

        for pass_node_index in 0..self.pass_nodes.len() {
            let pass_node_info = self.pass_nodes[pass_node_index].get_info();
            if pass_node_info.ref_count == 0 {
                continue;
            }

            for (attachment_index, attachment_info) in
                pass_node_info.attachments_infos.iter().enumerate()
            {
                let resource_node_info = self
                    .get_resource_node(attachment_info.texture_handle_index)
                    .get_info();

                let info =
                    self.virtual_resources[resource_node_info.virtual_resource_id].get_info();

                let last_pass_node_device_pass_id =
                    self.pass_nodes[info.last_pass_index.unwrap()].device_pass_id;

                if info.imported || resource_node_info.reader_count == 0 {
                    if pass_node_info.subpass {
                        if pass_node_info.device_pass_id != last_pass_node_device_pass_id {
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
                            resource_node_info.virtual_resource_id,
                            resource_node_info.version - 1,
                        )
                        .map(|node| node.get_info())
                    {
                        let write_pass_node_info = self.pass_nodes[new_version_resource_node_info
                            .pass_node_writer_index
                            .unwrap()]
                        .get_info();

                        if write_pass_node_info.device_pass_id == pass_node_info.device_pass_id {
                            self.pass_nodes[pass_node_index].attachments[attachment_index]
                                .store_op = StoreOp::Store;

                            if let Some(writer_attachment_index) = self.pass_nodes
                                [new_version_resource_node_info
                                    .pass_node_writer_index
                                    .unwrap()]
                            .get_render_target_attachment_index(
                                self,
                                new_version_resource_node_info.virtual_resource_id,
                            ) {
                                self.pass_nodes[new_version_resource_node_info
                                    .pass_node_writer_index
                                    .unwrap()]
                                .attachments[writer_attachment_index]
                                    .store_op = StoreOp::Discard
                            }
                        }
                    }
                }

                if attachment_info.load_op == LoadOp::Load {
                    let info = self.virtual_resources[resource_node_info.virtual_resource_id]
                        .get_mut_info();

                    info.never_loaded = false;
                }

                if attachment_info.store_op == StoreOp::Store {
                    let info = self.virtual_resources[resource_node_info.virtual_resource_id]
                        .get_mut_info();

                    info.never_stored = false;
                }

                resource_ids.push(resource_node_info.virtual_resource_id);
            }
        }

        //todo update memoryless and memorylessMSAA
        for resource_id in resource_ids.into_iter() {
            println!("{}", resource_id)
        }
    }

    pub fn get_resource_node(&self, index: usize) -> &ResourceNode {
        &self.resource_nodes[index]
    }

    pub fn get_resource_node_with_version(
        &self,
        index: usize,
        version: u8,
    ) -> Option<&ResourceNode> {
        if self.resource_nodes[index].version == version {
            Some(&self.resource_nodes[index])
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
                let resource = &mut self.virtual_resources[resource_node.virtual_resource_id];
                resource.get_mut_info().update_lifetime(pass_node);
            }

            //更新渲染节点吸入的资源节点所指向资源的生命周期
            for resource_index in pass_node.writes.iter() {
                let resource_node = &self.resource_nodes[*resource_index];
                let resource = &mut self.virtual_resources[resource_node.virtual_resource_id];
                let info = resource.get_mut_info();
                info.update_lifetime(pass_node);
                info.writer_count += 1;
            }

            pass_node.attachments.sort();
        }

        //更新pass_node中资源使用的索引顺序
        for resource_index in 0..self.virtual_resources.len() {
            let resource = &self.virtual_resources[resource_index];
            let info = resource.get_info();
            if info.first_pass_index.is_none() || info.last_pass_index.is_none() {
                continue;
            }

            let last_pass_index = info.last_pass_index.unwrap();
            let pass_node = &self.pass_nodes[last_pass_index];
            let has_attachment = pass_node
                .get_render_target_attachment(self, info.id)
                .is_some();

            if info.ref_count == 0 && !has_attachment {
                continue;
            }

            let first_pass_index = info.first_pass_index.unwrap();

            let first_pass_node = &mut self.pass_nodes[first_pass_index];
            first_pass_node.resource_request_array.push(info.id);

            let last_pass_node = &mut self.pass_nodes[last_pass_index];
            last_pass_node.resource_release_array.push(info.id);
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

        let mut resource_index_stack: Vec<usize> = vec![];

        //记录所有要被剔除的资源节点
        for (resource_index, resource_node) in self.resource_nodes.iter().enumerate() {
            if resource_node.reader_count == 0 && resource_node.pass_node_writer_index.is_some() {
                resource_index_stack.push(resource_index);
            }
        }

        //删除资源节点引用的pass_node计数
        while !resource_index_stack.is_empty() {
            let resource_node = &self.resource_nodes[resource_index_stack.pop().unwrap()];

            let pass_node_writer =
                &mut self.pass_nodes[resource_node.pass_node_writer_index.unwrap()];

            pass_node_writer.ref_count -= 1;

            //去除pass_node读取的资源节点引用
            if pass_node_writer.ref_count == 0 {
                for resource_index in pass_node_writer.reads.iter() {
                    let resource_node = &mut self.resource_nodes[*resource_index];
                    resource_node.reader_count -= 1;

                    if resource_node.reader_count == 0 {
                        resource_index_stack.push(*resource_index);
                    }
                }
            }
        }

        //更新资源节点对应的虚拟资源引用
        for resource_node in self.resource_nodes.iter() {
            let resource = &mut self.virtual_resources[resource_node.virtual_resource_id];
            resource.get_mut_info().ref_count += 1;
        }
    }

    pub fn merge_pass_nodes(&mut self) {
        let count = self.pass_nodes.len();
        let mut current_pass_id = 0;
        let mut last_pass_id;

        //获取最近且有效的pass node
        while current_pass_id < count {
            let pass_node = &self.pass_nodes[current_pass_id];

            if pass_node.ref_count != 0 {
                break;
            }
            current_pass_id += 1;
        }

        last_pass_id = current_pass_id;

        while {
            current_pass_id += 1;
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

                    prev_pass_node.id
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
                        [attachment_in_current_pass_node.texture_handle.index]
                        .reader_count;

                    let resource_node =
                        &mut self.resource_nodes[attachment_in_last_pass_node.texture_handle.index];

                    let write_count = self.virtual_resources[resource_node.virtual_resource_id]
                        .get_info()
                        .writer_count;

                    assert_eq!(write_count, 1);

                    self.virtual_resources[resource_node.virtual_resource_id]
                        .get_mut_info()
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
        DescriptorType: FrameResourceDescriptor + TypeEquals<Other = <<DescriptorType as FrameResourceDescriptor>::Resource as FrameResource>::Descriptor>,
    {
        let virtual_resource: Box<dyn VirtualResource> =
            Box::new(ResourceEntry::<DescriptorType::Resource>::new(
                self.virtual_resources.len(),
                name,
                TypeEquals::same(desc),
            ));

        let index = self.create_resource_node(virtual_resource);

        TypedHandle::new(index)
    }

    ///指向已存在的资源
    pub fn create_resource_node_with_id(&mut self, virtual_resource_id: usize) -> usize {
        let version = self.virtual_resources[virtual_resource_id]
            .get_info()
            .version;
        let index = self.resource_nodes.len();

        self.resource_nodes
            .push(ResourceNode::new(virtual_resource_id, version));

        index
    }

    pub fn create_resource_node(&mut self, virtual_resource: Box<dyn VirtualResource>) -> usize {
        let id = virtual_resource.get_info().id;
        let version = virtual_resource.get_info().version;
        self.virtual_resources.push(virtual_resource);

        let index = self.resource_nodes.len();

        self.resource_nodes.push(ResourceNode::new(id, version));

        index
    }
}

pub struct ResourceNode {
    pub virtual_resource_id: usize,
    version: u8,
    reader_count: u32,
    pub pass_node_writer_index: Option<usize>,
}

pub struct ResourceNodeInfo {
    pub virtual_resource_id: usize,
    pub version: u8,
    pub reader_count: u32,
    pub pass_node_writer_index: Option<usize>,
}

impl ResourceNode {
    pub fn get_info(&self) -> ResourceNodeInfo {
        ResourceNodeInfo {
            virtual_resource_id: self.virtual_resource_id,
            version: self.version,
            reader_count: self.reader_count,
            pass_node_writer_index: self.pass_node_writer_index,
        }
    }

    pub fn new(virtual_resource_id: usize, version: u8) -> Self {
        Self {
            virtual_resource_id,
            version,
            reader_count: 0,
            pass_node_writer_index: None,
        }
    }
}

pub type DynRenderFn = dyn FnOnce() -> Result<(), RendererError>;
