mod prelude;
use crate::prelude::*;

mod error;

mod utils;

mod core;
use crate::core::*;

/// Start of the projet by this function
fn main() -> Result<()> {
    println!("Hello, world!");

    let point = point::Builder::new().x(1.42).y(2.0).build()?;

    let point_two = point::Builder::new()
        .adress("144 rue du bosquet 34980 Saint Clement de riviere".to_string())
        .build()?;

    let point_four = point::Builder::new()
        .adress("2800 avenue des moulins".to_string())
        .build()?;

    println!("{:?}", point);
    println!("{:?}", point_two);
    let point_three = point::Builder::new()
        .adress("this adress don't exist bro".to_string())
        .build();

    println!("{:?}", point_three);

    let path = path::Builder::new()
        .point(point)
        .point(point_two)
        .point(point_four)
        .build()?;
    println!("{:?}", path);
    Ok(())
}
