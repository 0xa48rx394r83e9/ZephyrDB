use crate::storage::Storage;
use crate::types::Value;

pub enum QueryCondition {
    Equals(String, Value),
    NotEquals(String, Value),
    GreaterThan(String, Value),
    GreaterThanOrEqual(String, Value),
    LessThan(String, Value),
    LessThanOrEqual(String, Value),
}

pub struct Query {
    conditions: Vec<QueryCondition>,
}

impl Query {
    pub fn new() -> Self {
        Query {
            conditions: Vec::new(),
        }
    }

    pub fn add_condition(&mut self, condition: QueryCondition) {
        self.conditions.push(condition);
    }

    pub fn execute(&self, storage: &dyn Storage) -> Vec<Value> {
        let mut results = Vec::new();

        for (key, value_with_ttl) in storage.iter() {
            if self.matches(key, &value_with_ttl.value) {
                results.push(value_with_ttl.value.clone());
            }
        }

        results
    }

    fn matches(&self, key: &str, value: &Value) -> bool {
        for condition in &self.conditions {
            match condition {
                QueryCondition::Equals(ref k, ref v) => {
                    if key == k && value == v {
                        continue;
                    } else {
                        return false;
                    }
                }
                QueryCondition::NotEquals(ref k, ref v) => {
                    if key == k && value != v {
                        continue;
                    } else {
                        return false;
                    }
                }
                QueryCondition::GreaterThan(ref k, ref v) => {
                    if key == k && value > v {
                        continue;
                    } else {
                        return false;
                    }
                }
                QueryCondition::GreaterThanOrEqual(ref k, ref v) => {
                    if key == k && value >= v {
                        continue;
                    } else {
                        return false;
                    }
                }
                QueryCondition::LessThan(ref k, ref v) => {
                    if key == k && value < v {
                        continue;
                    } else {
                        return false;
                    }
                }
                QueryCondition::LessThanOrEqual(ref k, ref v) => {
                    if key == k && value <= v {
                        continue;
                    } else {
                        return false;
                    }
                }
            }
        }

        true
    }
}