use std::collections::HashMap;

use crate::{Texture, TextureDescriptor};

#[derive(Default, Debug)]
pub struct TransientResourceCache {
    textures: HashMap<TextureDescriptor, Vec<Texture>>,
}

impl TransientResourceCache {
    pub fn get_image(&mut self, desc: &TextureDescriptor) -> Option<Texture> {
        if let Some(entry) = self.textures.get_mut(desc) {
            entry.pop()
        } else {
            None
        }
    }

    pub fn insert_image(&mut self, desc: TextureDescriptor, resource: Texture) {
        if let Some(entry) = self.textures.get_mut(&desc) {
            entry.push(resource);
        } else {
            self.textures.insert(desc, vec![resource]);
        }
    }
}
