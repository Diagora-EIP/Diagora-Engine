use crate::prelude::*;
use crate::types::config_update::ConfigUpdate;
use crate::core::point;

pub fn update_itinary(args: ConfigUpdate) -> Result<()> {
    let mut points: Vec<point::Point> = Vec::new();

    for adress in args.address_path {
        let point = point::Builder::new().adress(adress.address).build()?;
        points.push(point)
    }
    println!("Not implemented yet");
    Ok(())
}