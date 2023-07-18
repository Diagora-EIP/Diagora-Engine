use serde::{Deserialize, Serialize};
use std::{env, fs};

use crate::prelude::*;
use crate::types::adress::Address;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub filepath: Option<String>,
    pub address: Vec<Address>,
    pub return_to_start: bool,
    pub start_adress: Address,
    pub itinary_day: Option<String>,
}

#[derive(Default)]
pub struct Builder {}

impl Builder {
    pub fn new() -> Self {
        Builder::default()
    }

    pub fn build(&self) -> Result<Config> {
        let args = self.get_args();
        if args[0] == "--json" || args[0] == "-j" {
            let content = fs::read_to_string(args[1].clone()).expect("Should Provide a valid file");
            let mut json: Config = serde_json::from_str(&content)?;
            json.filepath = Some(args[1].clone());
            return Ok(json);
        }
        return Err(Error::BadParameter());
    }

    fn get_args(&self) -> Vec<String> {
        let mut args: Vec<String> = env::args().collect();
        args.remove(0);
        args
    }
}
