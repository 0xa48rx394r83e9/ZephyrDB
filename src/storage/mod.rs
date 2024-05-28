mod btree;
mod hashmap;

pub use btree::BTreeStorage;
pub use hashmap::HashMapStorage;
use crate::types::{Value, ValueWithTTL};
use lz4::{Encoder, Decoder};
use std::collections::HashMap;

pub trait Storage: Send + Sync {
    fn insert(&mut self, key: &str, value: ValueWithTTL);
    fn get(&self, key: &str) -> Option<&ValueWithTTL>;
    fn remove(&mut self, key: &str) -> Option<ValueWithTTL>;
    fn iter(&self) -> Box<dyn Iterator<Item = (&String, &ValueWithTTL)> + '_>;
    fn remove_expired(&mut self);

    fn compress(&self, value: &ValueWithTTL) -> Vec<u8> {
        let mut encoder = Encoder::new(Vec::new());
        serde_json::to_writer(&mut encoder, value).unwrap();
        encoder.finish().unwrap()
    }

    fn decompress(&self, compressed: &[u8]) -> ValueWithTTL {
        let mut decoder = Decoder::new(compressed);
        serde_json::from_reader(&mut decoder).unwrap()
    }
}