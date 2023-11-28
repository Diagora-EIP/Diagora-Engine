use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]

/// Struct that represent an adress
pub struct Address {
    pub address: String,
    pub begin: Option<u64>,
    pub end: Option<u64>,
}
