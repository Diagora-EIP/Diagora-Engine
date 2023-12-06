use crate::error;
use crate::prelude::*;
use crate::types::config_create::ConfigCreate;
use crate::core::path;
use crate::core::point;
use crate::types::either::Either;
use std::fs::File;
use crate::utils::writer::write_in_output;

pub fn check_all_data_requirement(args: ConfigCreate) -> bool {
    true 
}

pub fn create_itinary(args: ConfigCreate) -> Result<()> {
    if check_all_data_requirement(args.clone()) {

    }
    let start_point = point::Builder::new()
        .adress(args.start_adress.address)
        .start_at(args.start_adress.start_at.unwrap())
        .build()?;
    let mut points: Vec<point::Point> = Vec::new();

    for adress in args.address {
        let point = point::Builder::new().adress(adress.address).build()?;
        points.push(point)
    }

    let path = path::Builder::new()
        .start_point(start_point)
        .points(points)
        .return_to_start(args.return_to_start)
        .build()?;
    let _ = write_in_output(Some(args.filepath.unwrap() + "_result.json"), &path);
    Ok(())
}