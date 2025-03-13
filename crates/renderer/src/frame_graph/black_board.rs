use std::collections::HashMap;

use crate::gfx_base::Handle;

pub struct BlackBoardKey(String);

impl BlackBoardKey {
    pub fn new(key: &str) -> Self {
        BlackBoardKey(key.to_string())
    }
}

//存储全局的索引
#[derive(Default)]
pub struct BlackBoard {
    handles: HashMap<String, Handle>,
}

impl BlackBoard {
    pub fn put(&mut self, key: &str, handle: Handle) {
        self.handles.insert(key.to_string(), handle);
    }

    pub fn get(&self, key: &str) -> Option<&Handle> {
        self.handles.get(key)
    }
}
