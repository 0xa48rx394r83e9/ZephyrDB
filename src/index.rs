use std::collections::HashMap;
use crate::types::Value;

pub struct Index {
    data: HashMap<String, Vec<String>>,
}

impl Index {
    pub fn new() -> Self {
        Index {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: &str, value: &Value) {
        let entry = self.data.entry(value.to_string()).or_insert_with(Vec::new);
        entry.push(key.to_string());
    }

    pub fn remove(&mut self, key: &str, value: &Value) {
        if let Some(entry) = self.data.get_mut(&value.to_string()) {
            entry.retain(|k| k != key);
            if entry.is_empty() {
                self.data.remove(&value.to_string());
            }
        }
    }

    pub fn get(&self, value: &Value) -> Option<&Vec<String>> {
        self.data.get(&value.to_string())
    }
}