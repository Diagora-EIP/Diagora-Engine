use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]

/// Struct that represent an adress
pub struct Address {
    pub address: String,
    pub begin: Option<String>,
    pub end: Option<String>,
    pub movable : Option<bool>
}
