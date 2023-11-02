use crate::prelude::*;
use crate::types::config_create::ConfigCreate;
use crate::core::path;
use crate::core::point;
use std::fs::File;

pub fn create_itinary(args: ConfigCreate) -> Result<()> {
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