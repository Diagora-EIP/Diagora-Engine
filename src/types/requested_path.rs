use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestedPath {
    code: String,
    pub routes: Vec<Route>,
    pub waypoints: Vec<Waypoint>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Route {
    pub legs: Vec<Leg>,
    pub weight_name: String,
    pub weight: f64,
    pub duration: f64,
    pub distance: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Leg {
    pub steps: Vec<Step>,
    pub summary: String,
    pub weight: f64,
    pub duration: f64,
    pub distance: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Step {
    pub geometry: String,
    pub maneuver: Maneuver,
    pub mode: Mode,
    pub driving_side: DrivingSide,
    pub name: String,
    pub intersections: Vec<Intersection>,
    pub weight: f64,
    pub duration: f64,
    pub distance: f64,
    #[serde(rename = "ref")]
    pub step_ref: Option<String>,
    pub rotary_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Intersection {
    out: Option<i64>,
    entry: Vec<bool>,
    bearings: Vec<i64>,
    location: Vec<f64>,
    #[serde(rename = "in")]
    intersection_in: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Maneuver {
    pub bearing_after: i64,
    bearing_before: i64,
    pub location: Vec<f64>,
    modifier: Option<DrivingSide>,
    #[serde(rename = "type")]
    maneuver_type: String,
    exit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Driving,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Waypoint {
    hint: String,
    distance: f64,
    name: String,
    location: Vec<f64>,
}
