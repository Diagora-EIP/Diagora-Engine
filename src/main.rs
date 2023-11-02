//! Entry point of the program

mod prelude;
use crate::prelude::*;
mod error;
mod types;
mod utils;
mod execution_flow;
mod core;
use crate::core::*;
use crate::types::either::Either;

use crate::execution_flow::create_itinary::create_itinary;

/// Start of the projet by this function
fn main() -> Result<()> {
    let args = execution_flow::args_handling::Builder::new().build()?;

    match args {
        Either::Left(config) => {
            create_itinary(config)?;
        }
        Either::Right(config) => {
            panic!("Not implemented yet")
        }
    }
    Ok(())
}
