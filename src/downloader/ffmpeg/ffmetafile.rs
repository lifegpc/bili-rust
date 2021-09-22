use std::clone::Clone;
use std::collections::HashMap;
use std::default::Default;

#[derive(Debug)]
pub struct FFMetaFile {
    pub basic: HashMap<String, String>,
    pub extra: HashMap<String, HashMap<String, String>>,
}

impl Clone for FFMetaFile {
    fn clone(&self) -> Self {
        Self {
            basic: self.basic.clone(),
            extra: self.extra.clone(),
        }
    }
}

impl Default for FFMetaFile {
    fn default() -> Self {
        Self {
            basic: HashMap::new(),
            extra: HashMap::new(),
        }
    }
}
