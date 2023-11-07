use crate::prelude::*;
use crate::types::config_update::ConfigUpdate;
use crate::core::path;
use crate::core::point;
use std::fs::File;


pub fn update_itinary(args: ConfigUpdate) -> Result<()> {
    let mut points: Vec<point::Point> = Vec::new();

    for adress in args.address_path {
        let point = point::Builder::new().adress(adress.address).build()?;
        points.push(point)
    }

    let addable_point = point::Builder::new().adress(args.addable_address.address).build()?;
    let start_point = point::Builder::new().adress(args.start_adress.address).build()?;

    let path = path::Builder::new()
        .points(points)
        .addable_point(addable_point).start_point(start_point)
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