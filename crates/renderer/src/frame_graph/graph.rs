use super::{
    FrameResource, FrameResourceDescriptor, TypeEquals,
    pass::PassNode,
    virtual_resources::{ResourceEntry, VirtualResource},
};
use crate::{RendererError, gfx_base::LoadOp, utils::IndexHandle};
use std::marker::PhantomData;

pub type StringHandle = IndexHandle<String, u32>;
pub type PassInsertPoint = u16;

#[derive(Debug, PartialEq, Eq)]
pub struct TypedHandle<ResourceType> {
    pub index: usize,
    _marker: PhantomData<ResourceType>,
}

impl<ResourceType: Ord> Ord for TypedHandle<ResourceType> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl<ResourceType: PartialEq> PartialOrd for TypedHandle<ResourceType> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl<ResourceType> TypedHandle<ResourceType> {
    const INVALID: usize = usize::MAX;
}

impl<ResourceType> Default for TypedHandle<ResourceType> {
    fn default() -> Self {
        Self {
            index: Self::INVALID,
            _marker: Default::default(),
        }
    }
}

impl<ResourceType> TypedHandle<ResourceType> {
    fn new(index: usize) -> Self {
        Self {
            index,
            _marker: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct FrameGraph {
    virtual_resources: Vec<Box<dyn VirtualResource>>,
    resource_nodes: Vec<ResourceNode>,
    pass_nodes: Vec<PassNode>,
    pub merge: bool,
}

impl FrameGraph {
    pub fn get_resource_node(&self, index: usize) -> &ResourceNode {
        &self.resource_nodes[index]
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
    pass_node_writer_index: Option<usize>,
}

impl ResourceNode {
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

mod test {
    use crate::frame_graph::{FrameResource, FrameResourceDescriptor};

    pub struct TestFrameResource;

    pub struct TestFrameResourceDescriptor;

    impl FrameResource for TestFrameResource {
        type Descriptor = TestFrameResourceDescriptor;
    }

    impl FrameResourceDescriptor for TestFrameResourceDescriptor {
        type Resource = TestFrameResource;
    }

    #[test]
    fn test_create() {
        use super::{FrameGraph, StringHandle};

        let mut frame_graph = FrameGraph::default();

        let desc = TestFrameResourceDescriptor;

        let name = StringHandle::new("test".to_string(), 1);

        let handle = frame_graph.create(name, desc);

        assert_eq!(handle.index, 0);
    }
}
