use serde::{Deserialize, Serialize};
use crate::types::adress::Address;

//: Stuct that represent the config file to use the program
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConfigCreate {
    pub filepath: Option<String>,
    pub address: Vec<Address>,
    pub return_to_start: bool,
    pub start_adress: Address,
    pub itinary_day: Option<String>,
}