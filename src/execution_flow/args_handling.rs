use std::{env, fs};

use crate::prelude::*;
use crate::types::config_update::ConfigUpdate;
use crate::types::either::Either;
use crate::types::config_create::ConfigCreate;

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
    pub fn build(&self) -> Result<Either<ConfigCreate, ConfigUpdate>> {
        let args = self.get_args();
        if args[0] == "--createItinary" || args[0] == "-c" {
            let content = fs::read_to_string(args[1].clone()).expect("Should Provide a valid file");
            let mut json: ConfigCreate = serde_json::from_str(&content)?;
            json.filepath = Some(args[1].clone());
            return Ok(Either::Left(json));
        } else if args[0] == "--updateItinary" || args[0] == "-u" {
            let content = fs::read_to_string(args[1].clone()).expect("Should Provide a valid file");
            let mut json: ConfigUpdate = serde_json::from_str(&content)?;
            json.filepath = Some(args[1].clone());
            return Ok(Either::Right(json));
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
