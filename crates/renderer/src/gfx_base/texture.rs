#[derive(Debug)]
pub struct Texture {
    desc: TextureDescriptor,
}

impl Texture {
    pub fn get_desc(&self) -> &TextureDescriptor {
        &self.desc
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TextureDescriptor;
