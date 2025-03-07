#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Texture {}

pub struct TextureDescriptor {}

impl FrameResource for Texture {
    type Descriptor = TextureDescriptor;
}

impl FrameResourceDescriptor for TextureDescriptor {
    type Resource = Texture;
}

///资源
pub trait FrameResource: 'static {
    type Descriptor: FrameResourceDescriptor;
}

///资源描述符
pub trait FrameResourceDescriptor {
    type Resource: FrameResource;
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
