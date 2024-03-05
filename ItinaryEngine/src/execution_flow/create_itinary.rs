
use itertools::Either;

use crate::prelude::*;
use crate::types::config_create::ConfigCreate;
use crate::core::path;
use crate::core::point;
use crate::types::error_message;
use crate::utils::writer::{write_in_output, write_error_output};
use std::collections::HashSet;


pub fn check_all_data_requirement(args: ConfigCreate) -> bool {
    if args.start_adress.start_at.is_none() {
        let error_message = error_message::ErrorMessage {
            code: 2,
            error: "Field missing".to_string(),
            reason: "Mandatory field start_at is missing".to_string(),
        };
        let _  = write_error_output(args.filepath, &error_message);
        return false;
    }
    return true;
}

pub fn check_duplicate_point(filepath: Option<String>, points: Vec<point::Point>) -> bool {
    let mut seen_points = HashSet::new();

    for point in &points {
        if !seen_points.insert(point) {
            let error = error_message::ErrorMessage {
                code: 3,
                error: "Duplicate point".to_string(),
                reason: "Duplication of the point: ".to_string() + &point.address.as_ref().unwrap(), 
            };
            let _ = write_error_output(filepath, &error);
            return true;
        }
    }

    false
}

fn check_time_gap(points: Vec<point::Point>) -> bool {
    for window in points.windows(2) {
        if let (Some(end1), Some(start2)) = (window[0].end_at, window[1].start_at) {
            let time_gap_hours = (start2 - end1).into_inner() / 3600.0;
            if time_gap_hours > 16.0 {
                return true;
            }
        }
    }

    false
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
        let point = point::Builder::new().adress(adress.address).start_at(adress.start_at).end_at(adress.end_at).build();
        match point {
            Ok(point) => {
                points.push(point)
            },
            Err(error) => {
                let error = error_message::ErrorMessage {
                    code: 2,
                    error: "Adress not valid".to_string(),
                    reason: error.to_string(),
                };
                let _ = write_error_output(args.filepath, &error);
                return Ok(());
            }
        }
    }
    if check_time_gap(points.clone()) {
        let error = error_message::ErrorMessage {
            code: 4,
            error: "Time gap too big".to_string(),
            reason: "There is more than 16 hours between timestamps".to_string(),
        };
        let _ = write_error_output(args.filepath, &error);
        return Ok(());
    }
    if check_duplicate_point(args.filepath.clone(), points.clone()) {
        return Ok(());
    }
    let path = path::Builder::new()
        .start_point(start_point)
        .points(points)
        .return_to_start(args.return_to_start)
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