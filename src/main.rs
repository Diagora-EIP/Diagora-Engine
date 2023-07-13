mod prelude;
use crate::prelude::*;

mod error;

mod types;

mod utils;

mod core;
use crate::core::*;

/// Start of the projet by this function
fn main() -> Result<()> {
    println!("Hello, world!");

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
    println!("{:?}", path);
    Ok(())
}
