use crate::prelude::*;
use reqwest;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Default)]
pub struct Builder {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub adress: Option<String>,
}

impl Builder {
    pub fn new() -> Self {
        Builder::default()
    }

    pub fn x(mut self, x: f32) -> Self {
        self.x = Some(x);
        self
    }

    pub fn y(mut self, y: f32) -> Self {
        self.y = Some(y);
        self
    }

    pub fn adress(mut self, adress: String) -> Self {
        self.adress = Some(adress);
        self
    }

    pub fn build(&self) -> Result<Point> {
        if self.x.is_none() || self.y.is_none() {
            let (x, y) = self.get_address()?;
        return Ok(Point { x, y });
        }
        let x = self.x.ok_or_else(|| Box::new(PointError("x".to_string())))?;
        let y = self.y.ok_or_else(|| Box::new(PointError("y".to_string())))?;
        Ok(Point { x, y })
    }

    fn get_address(&self) -> Result<(f32, f32)> {
        let client = reqwest::blocking::Client::builder()
        .user_agent("Diagora")
        .build()?;

        let url = format!(
            "https://nominatim.openstreetmap.org/search/?q={}&limit=5&format=json&addressdetails=1",
            self.adress.as_ref().ok_or_else(|| Box::new(PointError("adress".to_string())))?
        );
        let request = client.request(reqwest::Method::GET, &url);
        let response = request.send()?.text()?;
        let body: Vec<serde_json::Value> = serde_json::from_str(&response)?;
        let x = body[0]["lat"].as_str().unwrap().parse::<f32>().unwrap();
        let y = body[0]["lon"].as_str().unwrap().parse::<f32>().unwrap();

        Ok((x, y))
    }
}

#[derive(Debug)]
struct PointError(String);

impl fmt::Display for PointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl Error for PointError {}