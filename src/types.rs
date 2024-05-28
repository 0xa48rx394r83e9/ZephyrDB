use std::cmp::Ordering;
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Null, Value::Null) => Some(Ordering::Equal),
            (Value::Bool(a), Value::Bool(b)) => a.partial_cmp(b),
            (Value::Int(a), Value::Int(b)) => a.partial_cmp(b),
            (Value::Float(a), Value::Float(b)) => a.partial_cmp(b),
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValueWithTTL {
    pub value: Value,
    pub expires_at: Option<Instant>,
}

impl ValueWithTTL {
    pub fn new(value: Value, ttl: Option<Duration>) -> Self {
        let expires_at = ttl.map(|duration| Instant::now() + duration);
        ValueWithTTL { value, expires_at }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at.map(|expires| expires < Instant::now()).unwrap_or(false)
    }
}