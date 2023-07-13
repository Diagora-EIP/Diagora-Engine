use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestedPath {
    code: String,
    pub routes: Vec<Route>,
    waypoints: Vec<Waypoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    legs: Vec<Leg>,
    weight_name: String,
    weight: f64,
    pub duration: f64,
    distance: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Leg {
    steps: Vec<Step>,
    summary: String,
    weight: f64,
    duration: f64,
    distance: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    geometry: String,
    maneuver: Maneuver,
    mode: Mode,
    driving_side: DrivingSide,
    name: String,
    intersections: Vec<Intersection>,
    weight: f64,
    duration: f64,
    distance: f64,
    #[serde(rename = "ref")]
    step_ref: Option<String>,
    rotary_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrivingSide {
    Left,
    Right,
    #[serde(rename = "slight left")]
    SlightLeft,
    #[serde(rename = "slight right")]
    SlightRight,
    Straight,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Intersection {
    out: Option<i64>,
    entry: Vec<bool>,
    bearings: Vec<i64>,
    location: Vec<f64>,
    #[serde(rename = "in")]
    intersection_in: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Maneuver {
    bearing_after: i64,
    bearing_before: i64,
    location: Vec<f64>,
    modifier: Option<DrivingSide>,
    #[serde(rename = "type")]
    maneuver_type: String,
    exit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Driving,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Waypoint {
    hint: String,
    distance: f64,
    name: String,
    location: Vec<f64>,
}
