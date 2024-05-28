use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DatabaseError {
    StorageError(String),
    QueryError(String),
    SerializationError(serde_json::Error),
    IOError(std::io::Error),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            DatabaseError::QueryError(msg) => write!(f, "Query error: {}", msg),
            DatabaseError::SerializationError(err) => write!(f, "Serialization error: {}", err),
            DatabaseError::IOError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DatabaseError::SerializationError(err) => Some(err),
            DatabaseError::IOError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for DatabaseError {
    fn from(err: serde_json::Error) -> Self {
        DatabaseError::SerializationError(err)
    }
}

impl From<std::io::Error> for DatabaseError {
    fn from(err: std::io::Error) -> Self {
        DatabaseError::IOError(err)
    }
}