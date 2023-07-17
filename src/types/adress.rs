use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct Address {
    address: String,
    begin: Option<String>,
    end: Option<String>,
}
