use serde::{Deserialize, Serialize};
use std::env;

use crate::types::adress::Address;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    adress: Vec<Address>,
    start_adress: Address,
    return_to_start: bool,
    itinary_day: Option<String>,
}

#[derive(Default)]
pub struct Builder {
    adress: Option<Vec<Address>>,
    start_adress: Option<Address>,
    return_to_start: Option<bool>,
    itinary_day: Option<String>,
}

impl Builder {
    pub fn new() -> Self {
        Builder::default()
    }

    pub fn build(&self) -> Vec<String> {
        self.get_args()
    }

    fn get_args(&self) -> Vec<String> {
        let mut args: Vec<String> = env::args().collect();
        args.remove(0);
        args
    }
}
