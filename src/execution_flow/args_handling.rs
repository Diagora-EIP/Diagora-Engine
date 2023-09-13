use serde::{Deserialize, Serialize};
use std::{env, fs};

use crate::prelude::*;
use crate::types::adress::Address;

//: Stuct that represent the config file to use the program
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
    //! Init of the builder for args_handling
    pub fn new() -> Self {
        Builder::default()
    }

    /// Built the config by checkings the args and opening the file
    ///
    /// # Return
    /// 
    /// * Config - Return the config object file that will be used
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

    /// Get the args of the program
    ///
    /// # Return
    ///
    /// * Vec<String> - Return the args of the program
    fn get_args(&self) -> Vec<String> {
        let mut args: Vec<String> = env::args().collect();
        args.remove(0);
        args
    }
}
