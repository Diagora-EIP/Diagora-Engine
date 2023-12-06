use ordered_float::OrderedFloat;
use serde::{Serialize, Deserialize};




#[derive(Debug, PartialEq, Clone, Eq, Hash, Serialize, Deserialize)]
pub struct GraphicalPoint {
    pub x: OrderedFloat<f64>,
    pub y: OrderedFloat<f64>,
    pub time_elapsed: OrderedFloat<f64>,
}

pub struct Builder {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub time_elapsed: Option<f64>,
}



impl Builder {
    /// Init of builder
    pub fn new() -> Self {
        Builder {
            x: None,
            y: None,
            time_elapsed: None,
        }
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

    /// Init of timeElapsed value
    pub fn time_elapsed(mut self, time_elapsed: f64) -> Self {
        self.time_elapsed = Some(time_elapsed);
        self
    }

    /// Build of the Point
    ///
    /// If you provide a X and a Y this will create a classic point
    /// Else will invoke the get_adress function that will search coordinate
    ///
    /// # Return
    ///
    /// * Point - Return the point
    pub fn build(self) -> GraphicalPoint {
        GraphicalPoint {
            x: OrderedFloat(self.x.unwrap()),
            y: OrderedFloat(self.y.unwrap()),
            time_elapsed: OrderedFloat(self.time_elapsed.unwrap()),
        }
    }
}