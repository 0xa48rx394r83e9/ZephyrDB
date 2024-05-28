use crate::storage::{BTreeStorage, HashMapStorage};
use crate::query::Query;
use crate::types::{Value, ValueWithTTL};
use crate::error::DatabaseError;
use crate::index::Index;
use tokio::sync::RwLock;
use std::fs::File;
use std::io::{Write, Read};
use serde::{Serialize, Deserialize};
use chrono::Utc;

#[derive(Serialize, Deserialize)]
pub struct Database {
    storage: RwLock<Box<dyn Storage>>,
    index: RwLock<Index>,
}

impl Database {
    pub fn new(storage_type: &str) -> Self {
        let storage: Box<dyn Storage> = match storage_type {
            "btree" => Box::new(BTreeStorage::new()),
            "hashmap" => Box::new(HashMapStorage::new()),
            _ => panic!("Unsupported storage type"),
        };
        Database {
            storage: RwLock::new(storage),
            index: RwLock::new(Index::new()),
        }
    }

    pub async fn insert(&self, key: &str, value: Value, ttl: Option<Duration>) {
        let value_with_ttl = ValueWithTTL::new(value, ttl);
        let mut storage = self.storage.write().await;
        let mut index = self.index.write().await;
        storage.insert(key, value_with_ttl.clone());
        index.insert(key, &value_with_ttl.value);
    }

    pub async fn get(&self, key: &str) -> Option<Value> {
        let storage = self.storage.read().await;
        storage.get(key).cloned().map(|value_with_ttl| value_with_ttl.value)
    }

    pub async fn remove(&self, key: &str) {
        let mut storage = self.storage.write().await;
        let mut index = self.index.write().await;
        if let Some(value_with_ttl) = storage.remove(key) {
            index.remove(key, &value_with_ttl.value);
        }
    }

    pub async fn execute(&self, query: &Query) -> Vec<Value> {
        let storage = self.storage.read().await;
        query.execute(&*storage)
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<(), DatabaseError> {
        let serialized = serde_json::to_string(self)?;
        let mut file = File::create(file_path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn load_from_file(file_path: &str) -> Result<Self, DatabaseError> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let database: Database = serde_json::from_str(&contents)?;
        Ok(database)
    }

    pub fn begin_transaction(&mut self) -> Transaction {
        Transaction {
            database: self,
            committed: false,
        }
    }

    pub fn create_backup(&self, backup_path: &str) -> Result<(), DatabaseError> {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let backup_file = format!("{}/backup_{}.db", backup_path, timestamp);
        self.save_to_file(&backup_file)?;
        Ok(())
    }

    pub fn restore_from_backup(&mut self, backup_file: &str) -> Result<(), DatabaseError> {
        *self = Database::load_from_file(backup_file)?;
        Ok(())
    }
}

pub struct Transaction<'a> {
    database: &'a mut Database,
    committed: bool,
}

impl<'a> Transaction<'a> {
    pub async fn insert(&mut self, key: &str, value: Value, ttl: Option<Duration>) {
        self.database.insert(key, value, ttl).await;
    }

    pub async fn remove(&mut self, key: &str) {
        self.database.remove(key).await;
    }

    pub async fn commit(mut self) -> Result<(), DatabaseError> {
        self.committed = true;
        Ok(())
    }

    pub async fn rollback(self) -> Result<(), DatabaseError> {
        if !self.committed {
            println!("rollback");
        }
        Ok(())
    }
}

impl<'a> Drop for Transaction<'a> {
    fn drop(&mut self) {
        if !self.committed {
            let _ = tokio::runtime::Runtime::new().unwrap().block_on(self.rollback());
        }
    }
}