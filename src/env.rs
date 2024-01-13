use std::collections::HashMap;

use crate::object::Object;

pub struct Env<'a>(HashMap<&'a [u8], Object>);

impl<'a> Env<'a> {
    pub fn new() -> Self {
        Env(HashMap::new())
    }

    pub fn get(&self, key: &[u8]) -> Option<Object> {
        self.0.get(key).cloned()
    }

    pub fn set(&mut self, key: &'a [u8], value: Object) {
        self.0.insert(key, value);
    }
}
