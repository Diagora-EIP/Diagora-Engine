use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]

/// Struct that represent an adress
pub struct Address {
    pub address: String,
    pub start_at: Option<f64>,
    pub end_at: Option<f64>,
}
