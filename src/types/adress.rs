use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct Address {
    pub address: String,
    pub begin: Option<String>,
    pub end: Option<String>,
}
