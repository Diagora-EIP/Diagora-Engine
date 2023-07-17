mod prelude;
use crate::prelude::*;

mod error;

mod types;

mod utils;

mod execution_flow;

mod core;
use crate::core::*;

use std::fs::File;

/// Start of the projet by this function
fn main() -> Result<()> {
    let args = execution_flow::args_handling::Builder::new().build();

    println!("{:?}", args);

    let point_two = point::Builder::new()
        .adress("144 rue du bosquet 34980 Saint Clement de riviere".to_string())
        .build()?;

    let point_four = point::Builder::new()
        .adress("2800 avenue des moulins".to_string())
        .build()?;

    println!("{:?}", point_two);
    println!("{:?}", point_four);

    let path = path::Builder::new()
        .point(point_two)
        .point(point_four)
        .build()?;
    serde_json::to_writer_pretty(&File::create("test_file.json")?, &path)?;
    Ok(())
}
