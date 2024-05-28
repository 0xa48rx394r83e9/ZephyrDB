use std::collections::BTreeMap;
use crate::storage::Storage;
use crate::types::ValueWithTTL;

pub struct BTreeStorage {
    data: BTreeMap<String, Vec<u8>>,
}

impl BTreeStorage {
    pub fn new() -> Self {
        BTreeStorage {
            data: BTreeMap::new(),
        }
    }
}

impl Storage for BTreeStorage {
    fn insert(&mut self, key: &str, value: ValueWithTTL) {
        let compressed = self.compress(&value);
        self.data.insert(key.to_string(), compressed);
    }

    fn get(&self, key: &str) -> Option<&ValueWithTTL> {
        self.data.get(key).map(|compressed| {
            let value_with_ttl = self.decompress(compressed);
            &value_with_ttl
        })
    }

    fn remove(&mut self, key: &str) -> Option<ValueWithTTL> {
        self.data.remove(key).map(|compressed| self.decompress(&compressed))
    }

    fn iter(&self) -> Box<dyn Iterator<Item = (&String, &ValueWithTTL)> + '_> {
        let iter = self.data.iter().map(|(key, compressed)| {
            let value_with_ttl = self.decompress(compressed);
            (key, &value_with_ttl)
        });
        Box::new(iter)
    }

    fn remove_expired(&mut self) {
        self.data.retain(|_, compressed| {
            let value_with_ttl = self.decompress(compressed);
            !value_with_ttl.is_expired()
        });
    }
}