use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorMessage {
    pub code: i64,
    pub error: String,
    pub reason: String,
}