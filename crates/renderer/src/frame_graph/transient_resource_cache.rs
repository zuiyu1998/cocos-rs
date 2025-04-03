use std::collections::HashMap;

use crate::{Texture, TextureInfo};

#[derive(Default, Debug)]
pub struct TransientResourceCache {
    textures: HashMap<TextureInfo, Vec<Texture>>,
}

impl TransientResourceCache {
    pub fn get_image(&mut self, desc: &TextureInfo) -> Option<Texture> {
        if let Some(entry) = self.textures.get_mut(desc) {
            entry.pop()
        } else {
            None
        }
    }

    pub fn insert_image(&mut self, desc: TextureInfo, resource: Texture) {
        if let Some(entry) = self.textures.get_mut(&desc) {
            entry.push(resource);
        } else {
            self.textures.insert(desc, vec![resource]);
        }
    }
}
