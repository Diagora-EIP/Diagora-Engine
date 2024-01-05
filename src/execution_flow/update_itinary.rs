use itertools::Either;

use crate::prelude::*;
use crate::types::config_update::ConfigUpdate;
use crate::core::path;
use crate::core::point;
use crate::utils::writer::{write_in_output, write_error_output};



pub fn update_itinary(args: ConfigUpdate) -> Result<()> {
    let mut points: Vec<point::Point> = Vec::new();

    for adress in args.address {
        let point = point::Builder::new().adress(adress.address).start_at(adress.start_at).end_at(adress.end_at).build()?;
        points.push(point)
    }
    let addable_point = point::Builder::new().adress(args.addable_address.address).start_at(args.addable_address.start_at).end_at(args.addable_address.end_at).build()?;
    let start_point = point::Builder::new().adress(args.start_adress.address).start_at(args.start_adress.start_at).end_at(args.start_adress.end_at).build()?;
    let path = path::Builder::new()
        .points(points)
        .addable_point(addable_point).start_point(start_point)
        .build()?;
    match path {
        Either::Left(path) => {
            let _ = write_in_output(Some(args.filepath.unwrap() + "_result.json"), &path);
        }
        Either::Right(error) => {
            let _ = write_error_output(args.filepath, &error);
        }
    }
    Ok(())
}