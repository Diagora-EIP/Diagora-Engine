use crate::error;
use crate::prelude::*;
use crate::types::config_create::ConfigCreate;
use crate::core::path;
use crate::core::point;
use crate::utils::writer::{write_in_output, write_error_output};

pub fn check_all_data_requirement(args: ConfigCreate) -> bool {
    if args.start_adress.start_at.is_none() {
        let _  = write_error_output(args.filepath, "start_at is missing");
        return false;
    }
    return true;
}

pub fn create_itinary(args: ConfigCreate) -> Result<()> {
    if !check_all_data_requirement(args.clone()) {
        return Ok(());
    }
    let start_point = point::Builder::new()
        .adress(args.start_adress.address)
        .start_at(args.start_adress.start_at)
        .build()?;
    let mut points: Vec<point::Point> = Vec::new();

    for adress in args.address {
        let point = point::Builder::new().adress(adress.address).start_at(adress.start_at).end_at(adress.end_at).build()?;
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