use crate::prelude::*;
use crate::utils::http;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

/// A Geometrical point that indicate a place on earth
#[derive(Debug, PartialEq, Clone, Eq, Hash, Serialize, Deserialize)]
pub struct Point {
    pub x: OrderedFloat<f64>,
    pub y: OrderedFloat<f64>,
    pub address: Option<String>,
    pub timeto_go: Option<OrderedFloat<f64>>,
    pub time_to_stay: Option<OrderedFloat<f64>>,
}

/// Builder of a Point
#[derive(Default)]
pub struct Builder {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub adress: Option<String>,
    pub timeto_go: Option<OrderedFloat<f64>>,
    pub time_to_stay: Option<OrderedFloat<f64>>,
}

impl Builder {
    /// Init of builder
    pub fn new() -> Self {
        Builder::default()
    }

    /// Init of x value
    pub fn x(mut self, x: f64) -> Self {
        self.x = Some(x);
        self
    }

    /// Init of y value
    pub fn y(mut self, y: f64) -> Self {
        self.y = Some(y);
        self
    }

    /// Init of adress value
    pub fn adress(mut self, adress: String) -> Self {
        self.adress = Some(adress);
        self
    }

    /// Init of timetoGo value
    pub fn timeto_go(mut self, timeto_go: f64) -> Self {
        self.timeto_go = Some(OrderedFloat(timeto_go));
        self
    }

    /// Init of time_to_stay value
    pub fn time_to_stay(mut self, time_to_stay: f64) -> Self {
        self.time_to_stay = Some(OrderedFloat(time_to_stay));
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
            return Ok(Point {
                x: OrderedFloat(x),
                y: OrderedFloat(y),
                address: self.adress.clone(),
                timeto_go: self.timeto_go.clone(),
                time_to_stay: self.time_to_stay.clone(),
            });
        }
        let x = self
            .x
            .ok_or_else(|| Error::PointError("No x point".to_string()))?;
        let y = self
            .y
            .ok_or_else(|| Error::PointError("No y point".to_string()))?;
        Ok(Point {
            x: OrderedFloat(x),
            y: OrderedFloat(y),
            address: None,
            timeto_go: self.timeto_go.clone(),
            time_to_stay: self.time_to_stay.clone(),
        })
    }

    /// Functiont that will use the adress to search coordinate
    ///
    ///
    /// # Return
    ///
    /// * (f64, f32) - A Tuple of point
    ///
    fn get_address(&self) -> Result<(f64, f64)> {
        let client = http::Builder::new()
            .user_agent("Diagora".to_string())
            .build()?;

        let url =
            format!(
            "https://nominatim.openstreetmap.org/search?q={}&limit=5&format=json&addressdetails=1",
            self.adress.as_ref().ok_or_else(|| Error::PointError("No Adress provide".to_string()))?
        );
        let response = client.get(url)?;
        let body: Vec<serde_json::Value> = serde_json::from_str(&response)?;
        if body.len() <= 0 {
            return Err(Error::PointError(
                "Adress not valid provide a valide Adress".to_string(),
            ));
        }
        let x = body[0]["lon"].as_str().unwrap().parse::<f64>().unwrap();
        let y = body[0]["lat"].as_str().unwrap().parse::<f64>().unwrap();

        Ok((x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_should_be_created_with_coordinate_given() {
        let point = Builder::new().x(5.5).y(1.4).build().unwrap();
        assert_eq!(
            point,
            Point {
                x: OrderedFloat(5.5),
                y: OrderedFloat(1.4),
                address: None,
                timeto_go: None,
                time_to_stay: None,
            }
        )
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
                y: OrderedFloat(43.6808855),
                x: OrderedFloat(3.8425387004424802),
                address: Some("144 rue du bosquet 34980 Saint Clement de riviere".to_string()),
                timeto_go: None,
                time_to_stay: None,

            }
        )
    }

    #[test]
    #[should_panic]
    fn point_should_return_an_error() {
        Builder::new()
            .adress("this adress don't exist bro".to_string())
            .build()
            .unwrap();
    }
}
