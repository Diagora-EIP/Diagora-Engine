use serde::{Deserialize, Serialize};
use crate::types::adress::Address;

//: Stuct that represent the config file to use the program
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConfigUpdate {
    pub filepath: Option<String>,
    pub address_path: Vec<Address>,
    pub addable_address: Vec<Address>,
}