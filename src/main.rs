//! Entry point of the program

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
    let args = execution_flow::args_handling::Builder::new().build()?;

    let start_point = point::Builder::new()
        .adress(args.start_adress.address).movable(false)
        .build()?;

    let mut points: Vec<point::Point> = Vec::new();

    for adress in args.address {
        let point = point::Builder::new().adress(adress.address).movable(adress.movable.is_some()).build()?;
        points.push(point)
    }

    let path = path::Builder::new()
        .start_point(start_point)
        .points(points)
        .return_to_start(args.return_to_start)
        .build()?;
    serde_json::to_writer_pretty(
        &File::create(args.filepath.clone().expect("No filepath Provided") + "_result.json")?,
        &path,
    )?;
    println!(
        "{:?}",
        args.filepath.expect("No filepath Provided") + "_result.json"
    );
    Ok(())
}
