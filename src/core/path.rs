//! The path module is used to create a path between multiple points

use itertools::Itertools;
use crate::core::point;
use crate::point::Point;
use crate::prelude::*;
use crate::types::requested_path;
use crate::utils::http;
use serde::{Deserialize, Serialize};

/// Array of Point with detailed road
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Path {
    pub start_point: Point,
    pub return_to_start: bool,
    pub points: Vec<Point>,
    pub road: Vec<Point>,
}

/// Builder of Path
#[derive(Default)]
pub struct Builder {
    pub points: Vec<Point>,
    pub return_to_start: bool,
    pub start_point: Option<Point>,
}

impl Builder {
    /// Init of builder
    pub fn new() -> Self {
        Builder {
            points: Vec::new(),
            return_to_start: false,
            start_point: None,
        }
    }

    /// Modification of all the array using a another array
    ///
    /// # Arguments
    ///
    /// * `points` - Array of Point
    ///
    /// # Return
    ///
    /// * Self - Return the builder
    pub fn points(mut self, points: Vec<Point>) -> Self {
        self.points = points;
        self
    }

    /// Add of a point for next calculation
    ///
    /// # Arguments
    ///
    /// * `point` - Point that will be added
    ///
    /// # Return
    ///
    /// * Self - Return the builder
    pub fn point(mut self, point: Point) -> Self {
        self.points.push(point);
        self
    }

    /// Modification of the start point
    ///
    /// # Arguments
    ///
    /// * `start_point` - Point that will be the start point
    ///
    /// # Return
    ///
    /// * Self - Return the builder
    pub fn start_point(mut self, start_point: Point) -> Self {
        self.start_point = Some(start_point);
        self
    }

    /// Modification of the return_to_start
    ///
    /// # Arguments
    ///
    /// * `return_to_start` - Bool that will be the return_to_start
    ///
    /// # Return
    ///
    /// * Self - Return the builder

    pub fn return_to_start(mut self, return_to_start: bool) -> Self {
        self.return_to_start = return_to_start;
        self
    }

    /// Building of a Path
    ///
    /// # Return
    ///
    /// * Path - Return the path
    pub fn build(&self) -> Result<Path> {
        Ok(self.create_best_path()?)
    }

    /// Create the best path by creating all the possible path and compare them
    ///
    /// # Return
    ///
    /// * Path - Return the best path
    fn create_best_path(&self) -> Result<Path> {
        let client = http::Builder::new()
            .user_agent("Diagora".to_string())
            .build()?;
        let mut best_path: Vec<Point> = Vec::new();
        let mut best_duration: f64 = 100000000.0;
        let mut best_body: Option<requested_path::RequestedPath> = None;

        for mut perm in self.points.iter().permutations(self.points.len()).unique() {
            perm.insert(0, self.start_point.as_ref().unwrap());

            if self.return_to_start {
                perm.push(self.start_point.as_ref().unwrap());
            }
            let url = self.create_url_path(&perm);
            let response = client.clone().get(url)?;
            let body: requested_path::RequestedPath = serde_json::from_str(&response)?;
            let time = body.routes[0].duration;
            if best_duration > time {
                best_duration = time;
                best_path = perm.into_iter().cloned().collect();
                best_body = Some(body);
            }
        }
        if best_path.is_empty() {
            return Err(Error::PathError("No best path found".to_string()));
        }
        Ok(Path {
            points: best_path,
            road: self.get_graphical_path(best_body.unwrap()),
            start_point: self.start_point.clone().unwrap(),
            return_to_start: false,
        })
    }

    /// Create the url for the request
    ///
    /// # Arguments
    ///
    /// * `points` - Array of Point
    ///
    /// # Return
    ///
    /// * String - Return the url
    fn create_url_path(&self, points: &Vec<&Point>) -> String {
        let format_point: String = points
            .into_iter()
            .map(|point| point.x.to_string() + "," + &point.y.to_string())
            .join(";");
        let url = format!("https://routing.openstreetmap.de/routed-car/route/v1/driving/{}?overview=false&alternatives=true&steps=true", format_point);
        url
    }

    /// Get the graphical path for gps use
    ///
    /// # Arguments
    ///
    /// * `body` - Body of the request
    /// 
    /// # Return
    /// 
    /// * Vec<Point> - Return the graphical path
    fn get_graphical_path(&self, body: requested_path::RequestedPath) -> Vec<Point> {
        let mut roads: Vec<Point> = Vec::new();
        let road = &body.routes[0].legs[0];

        for step in road.steps.clone() {
            let point = point::Builder::new()
                .x(step.maneuver.location[0])
                .y(step.maneuver.location[1])
                .build();
            roads.push(point.unwrap())
        }
        return roads;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point;

    #[test]
    fn creation_of_path_using_multiple_point() -> Result<()> {
        let point = point::Builder::new()
            .adress("144 rue du bosquet 34980 Saint Clement de riviere".to_string())
            .build()?;

        let point_two = point::Builder::new()
            .adress("2800 avenue des moulins".to_string())
            .build()?;

        let mut vector_test = Vec::new();
        vector_test.push(point.clone());
        vector_test.push(point_two.clone());
        let path = Builder::new().start_point(point).point(point_two).build()?;
        assert_eq!(path.points, vector_test);
        Ok(())
    }
}
