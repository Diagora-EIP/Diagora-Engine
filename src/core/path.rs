//! The path module is used to create a path between multiple points

use itertools::{Itertools, Either};
use ordered_float::OrderedFloat;
use crate::core::point;
use crate::point::Point;
use crate::prelude::*;
use crate::types::requested_path;
use crate::types::graphical_point::{GraphicalPoint, Builder as GraphicalPointBuilder};
use crate::utils::http;
use serde::{Deserialize, Serialize};

/// Array of Point with detailed road
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Path {
    pub return_to_start: bool,
    pub points: Vec<Point>,
    pub road: Vec<GraphicalPoint>,
}

/// Builder of Path
#[derive(Default)]
pub struct Builder {
    pub points: Vec<Point>,
    pub addable_point: Option<Point>,
    pub return_to_start: bool,
    pub start_point: Option<Point>,
}

impl Builder {
    /// Init of builder
    pub fn new() -> Self {
        Builder {
            points: Vec::new(),
            addable_point: None,
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

    pub fn addable_point(mut self, addable_point: Point) -> Self {
        self.addable_point = Some(addable_point);
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
    pub fn build(&self) -> Result<Either<Path, String>> {
        // if self.addable_point.is_some() {
        //     return Ok(self.add_a_point_to_path()?);
        // }
        Ok(self.create_best_path()?)
    }

    /// Create the best path by creating all the possible path and compare them
    ///
    /// # Return
    ///
    /// * Path - Return the best path
    fn create_best_path(&self) -> Result<Either<Path, String>> {
        let client = http::Builder::new()
            .user_agent("Diagora".to_string())
            .build()?;
        let mut best_path: Vec<Point> = Vec::new();
        let mut best_duration: OrderedFloat<f64> = OrderedFloat(f64::MAX);
        let mut best_body: Option<requested_path::RequestedPath> = None;

        for mut perm in self.points.iter().permutations(self.points.len()).unique() {
            perm.insert(0, self.start_point.as_ref().unwrap());

            if self.return_to_start {
                perm.push(self.start_point.as_ref().unwrap());
            }
            let url = self.create_url_path_itirator(&perm);
            let response = client.clone().get(url)?;
            let body: requested_path::RequestedPath = serde_json::from_str(&response)?;
            let time = self.calculate_time_with_fix_hour_point(&perm.clone().into_iter().cloned().collect(), body.clone(), self.start_point.as_ref().unwrap().start_at.unwrap());

            if best_duration > time && self.verify_time(&perm.clone().into_iter().cloned().collect(), body.clone()) {
                best_duration = time;
                best_path = perm.into_iter().cloned().collect();
                best_body = Some(body);
            }
        }
        if best_path.is_empty() {
            return Ok(Either::Right("No best path found".to_string()));
        }
        Ok(Either::Left(Path {
            points: self.apply_time_to_best_path(&best_path, best_body.clone().unwrap(), self.start_point.as_ref().unwrap().start_at.unwrap()),
            road: self.get_graphical_path(best_body.unwrap()),
            return_to_start: false,
        }))
    }

    fn verify_time(&self, path: &Vec<Point>, body: requested_path::RequestedPath) -> bool {
        let path = self.apply_time_to_best_path(path, body, self.start_point.as_ref().unwrap().start_at.unwrap());
        for (i, point) in path.iter().enumerate() {
            if point.end_at.is_some() && point.end_at.unwrap() < point.start_at.unwrap() {
                return false;
            }
            if i + 1 < path.len() && point.end_at.is_some() && path[i + 1].start_at.unwrap() < point.end_at.unwrap() {
                return false;
            }
            if point.arrive_at.unwrap() > point.start_at.unwrap() {
                return false;
            }
        }
        return true;
    }

    fn calculate_time_with_fix_hour_point(&self, path: &Vec<Point>, body: requested_path::RequestedPath, hour_start: OrderedFloat<f64>) -> OrderedFloat<f64> {
        let road = &body.routes[0];
        let mut path = path.clone();
        path[0].arrive_at = Some(hour_start);
        path[0].start_at = Some(hour_start);
        for (i, leg) in road.legs.clone().iter().enumerate() {
            if path[i].end_at.is_some() {
                let proposed_arrival = path[i].end_at.unwrap() + OrderedFloat(leg.duration);
                path[i + 1].arrive_at = Some(proposed_arrival.min(path[i + 1].arrive_at.unwrap_or(proposed_arrival)));
            } else {
                path[i + 1].arrive_at = Some(OrderedFloat(leg.duration) + path[i].arrive_at.unwrap());
            }
            if path[i + 1].start_at.is_none() {
                path[i + 1].start_at = path[i + 1].arrive_at.clone();
            }
        }
        let total_time = path[path.len() - 1].arrive_at.unwrap() - hour_start;
        return total_time;
    }

    fn add_a_point_to_path(&self) -> Result<Path> {
        let client = http::Builder::new()
            .user_agent("Diagora".to_string())
            .build()?;
        let mut best_path: Vec<Point> = Vec::new();
        let mut best_duration: f64 = 100000000.0;
        let mut best_body: Option<requested_path::RequestedPath> = None;
    
        // loop of the size of the Vector Point and add the addable point at each position of the vector verifying the best path
        for i in 0..self.points.len() {
            let mut perm = self.points.clone();
            perm.insert(i, self.addable_point.as_ref().unwrap().clone());
            perm.insert(0, self.start_point.as_ref().unwrap().clone());
    
            if self.return_to_start {
                perm.push(self.start_point.as_ref().unwrap().clone());
            }
            
            let url = self.create_url_path(perm.clone()); // Clone perm just before using it
            let response = client.clone().get(url)?;
            let body: requested_path::RequestedPath = serde_json::from_str(&response)?;
            let time = body.routes[0].duration;
    
            if best_duration > time {
                best_duration = time;
                best_path = perm;
                best_body = Some(body);
            }
        }
    
        if best_path.is_empty() {
            return Err(Error::PathError("No best path found".to_string()));
        }

        Ok(Path {
            points: best_path,
            road: self.get_graphical_path(best_body.unwrap()),
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
    fn create_url_path_itirator(&self, points: &Vec<&Point>) -> String {
        let format_point: String = points
            .into_iter()
            .map(|point| point.x.to_string() + "," + &point.y.to_string())
            .join(";");
        let url = format!("https://routing.openstreetmap.de/routed-car/route/v1/driving/{}?overview=false&alternatives=true&steps=true", format_point);
        url
    }


    fn create_url_path(&self, points: Vec<Point>) -> String {
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
    fn get_graphical_path(&self, body: requested_path::RequestedPath) -> Vec<GraphicalPoint> {
        let mut roads: Vec<GraphicalPoint> = Vec::new();
        let road = &body.routes[0];

        for leg in road.legs.clone() {
            for step in leg.steps.clone() {
                let graphical_point = GraphicalPointBuilder::new()
                    .x(step.maneuver.location[0])
                    .y(step.maneuver.location[1])
                    .time_elapsed(step.duration)
                    .build();
                roads.push(graphical_point)
            }
        }
        return roads;
    }

    fn apply_time_to_best_path(&self, best_path: &Vec<Point>, best_body: requested_path::RequestedPath, hour_start: OrderedFloat<f64>) -> Vec<Point> {
        let road = &best_body.routes[0];
        let mut best_path = best_path.clone();
        best_path[0].arrive_at = Some(hour_start);
        best_path[0].start_at = Some(hour_start);
        for (i, leg) in road.legs.clone().iter().enumerate() {
            if best_path[i].end_at.is_some() {

                best_path[i + 1].arrive_at = Some(best_path[i].end_at.unwrap() + OrderedFloat(leg.duration));
            } else {
                best_path[i + 1].arrive_at = Some(OrderedFloat(leg.duration) + best_path[i].arrive_at.unwrap());
            }
            if best_path[i + 1].start_at.is_none() {
                best_path[i + 1].start_at = best_path[i + 1].arrive_at.clone();
            }
        }
        return best_path.clone();
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
        Ok(())
    }
}
