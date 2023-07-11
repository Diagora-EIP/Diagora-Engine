use crate::prelude::*;
use crate::utils::http;
use std::error::Error;
use std::fmt;

/// A Geometrical point that indicate a place on earth
#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

/// Builder of a Point
#[derive(Default)]
pub struct Builder {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub adress: Option<String>,
}

impl Builder {
    /// Init of builder
    pub fn new() -> Self {
        Builder::default()
    }

    /// Init of x value
    pub fn x(mut self, x: f32) -> Self {
        self.x = Some(x);
        self
    }

    /// Init of y value
    pub fn y(mut self, y: f32) -> Self {
        self.y = Some(y);
        self
    }

    /// Init of adress value
    pub fn adress(mut self, adress: String) -> Self {
        self.adress = Some(adress);
        self
    }

    /// Build of the Point
    ///
    /// If you provide a X and a Y this will create a classic point
    /// Else will invoke the get_adress function that will search coordinate
    ///
    /// # Return
    ///
    /// * Point - The created Point
    ///
    pub fn build(&self) -> Result<Point> {
        if self.x.is_none() || self.y.is_none() {
            let (x, y) = self.get_address()?;
            return Ok(Point { x, y });
        }
        let x = self
            .x
            .ok_or_else(|| Box::new(PointError("x".to_string())))?;
        let y = self
            .y
            .ok_or_else(|| Box::new(PointError("y".to_string())))?;
        Ok(Point { x, y })
    }

    /// Functiont that will use the adress to search coordinate
    ///
    ///
    /// # Return
    ///
    /// * (f32, f32) - A Tuple of point
    ///
    fn get_address(&self) -> Result<(f32, f32)> {
        let client = http::Builder::new()
            .user_agent("Diagora".to_string())
            .build()?;

        let url =
            format!(
            "https://nominatim.openstreetmap.org/search/?q={}&limit=5&format=json&addressdetails=1",
            self.adress.as_ref().ok_or_else(|| Box::new(PointError("adress".to_string())))?
        );
        let body = client.get(url)?;
        let x = body[0]["lat"].as_str().unwrap().parse::<f32>().unwrap();
        let y = body[0]["lon"].as_str().unwrap().parse::<f32>().unwrap();

        Ok((x, y))
    }
}

#[derive(Debug)]
struct PointError(String);

impl fmt::Display for PointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error in Point: {}", self.0)
    }
}

impl Error for PointError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_should_be_created_with_coordinate_given() {
        let point = Builder::new().x(5.5).y(1.4).build().unwrap();
        assert_eq!(point, Point { x: 5.5, y: 1.4 })
    }

    #[test]
    fn point_should_give_coordinate_by_adress() {
        let point = Builder::new()
            .adress("144 rue du bosquet 34980 Saint Clement de riviere".to_string())
            .build()
            .unwrap();
        assert_eq!(
            point,
            Point {
                x: 43.680885,
                y: 3.8425386
            }
        )
    }
}
