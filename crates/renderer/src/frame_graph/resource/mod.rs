mod texture;

use std::{fmt::Debug, hash::Hash, sync::Arc};

use crate::{CommandBuffer, Texture, TextureDescriptor, gfx_base::TypeHandle};

use super::PassNode;

pub trait ResourceCreator: Clone {
    fn create(&self, desc: &AnyFGResourceDescriptor) -> AnyFGResource;
    fn get_command_buffer(&self) -> CommandBuffer;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AnyFGResourceDescriptor {
    Texture(TextureDescriptor),
}

#[derive(Debug)]
pub enum AnyFGResource {
    OwnedTexture(Texture),
    ImportedTexture(Arc<Texture>),
}

pub trait FGResource: 'static + Debug {
    type Descriptor: FGResourceDescriptor;
}

pub trait FGResourceDescriptor:
    'static + Clone + Hash + Eq + Debug + Into<AnyFGResourceDescriptor>
{
    type Resource: FGResource;
}

pub trait TypeEquals {
    type Other;
    fn same(value: Self) -> Self::Other;
}

impl<T: Sized> TypeEquals for T {
    type Other = Self;
    fn same(value: Self) -> Self::Other {
        value
    }
}

pub struct VirtualResource {
    pub info: ResourceInfo,
    pub state: VirtualResourceState,
}

impl VirtualResource {
    pub fn setup<ResourceType: FGResource>(
        name: &str,
        handle: TypeHandle<VirtualResource>,
        desc: ResourceType::Descriptor,
    ) -> VirtualResource {
        VirtualResource {
            state: VirtualResourceState::Setup(desc.into()),
            info: ResourceInfo::new(name, handle),
        }
    }
}

pub enum ImportedVirtualResource {
    Texture(Arc<Texture>),
}

pub struct ImportedVirtualResourceState {
    pub desc: AnyFGResourceDescriptor,
    pub resource: ImportedVirtualResource,
}

pub enum VirtualResourceState {
    Setup(AnyFGResourceDescriptor),
    Imported(ImportedVirtualResourceState),
}

///记录资源被使用的必要信息
#[derive(Clone)]
pub struct ResourceInfo {
    ///唯一的资源名称
    pub name: String,
    ///资源索引
    pub handle: TypeHandle<VirtualResource>,
    /// 资源版本
    version: u32,
    ///首次使用此资源的渲染节点
    pub first_pass_node_handle: Option<TypeHandle<PassNode>>,
    ///最后使用此资源的渲染节点
    pub last_pass_node_handle: Option<TypeHandle<PassNode>>,
}

impl ResourceInfo {
    pub fn new(name: &str, handle: TypeHandle<VirtualResource>) -> Self {
        ResourceInfo {
            name: name.to_string(),
            handle,
            version: 0,
            first_pass_node_handle: None,
            last_pass_node_handle: None,
        }
    }
}

impl ResourceInfo {
    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn new_version(&mut self) {
        self.version += 1
    }

    pub fn update_lifetime(&mut self, handle: TypeHandle<PassNode>) {
        if self.first_pass_node_handle.is_none() {
            self.first_pass_node_handle = Some(handle);
        }

        self.last_pass_node_handle = Some(handle)
    }
}
