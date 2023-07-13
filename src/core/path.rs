use itertools::Itertools;

use crate::point::Point;
use crate::prelude::*;

/// Array of Point with detailed road
#[derive(Debug, PartialEq)]
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
    pub return_to_start: Option<bool>,
    pub start_point: Option<Point>,
}

impl Builder {
    /// Init of builder
    pub fn new() -> Self {
        Builder {
            points: Vec::new(),
            return_to_start: Some(false),
            start_point: None,
        }
    }

    /// Modification of all the array using a another array
    pub fn points(mut self, points: Vec<Point>) -> Self {
        self.points = points;
        self
    }

    /// Add of a point for next calculation
    pub fn point(mut self, point: Point) -> Self {
        self.points.push(point);
        self
    }

    /// Building of a Path
    pub fn build(&self) -> Result<Path> {
        Ok(Path {
            points: self.points.clone(),
            road: Vec::new(),
            start_point: self.points[0],
            return_to_start: false,
        })
    }

    fn create_best_path(&self) -> Result<Path> {
        for perm in self.points.iter().permutations(self.points.len()).unique() {
            println!("{:?}", perm);
        }
        Ok(Path {
            points: self.points.clone(),
            road: Vec::new(),
            start_point: self.points[0],
            return_to_start: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point;

    #[test]
    fn creation_of_path_using_multiple_point() -> Result<()> {
        let point = point::Builder::new().x(1.42).y(2.0).build()?;

        let point_two = point::Builder::new()
            .adress("144 rue du bosquet 34980 Saint Clement de riviere".to_string())
            .build()?;

        let mut vector_test = Vec::new();
        vector_test.push(point);
        vector_test.push(point_two);
        let path = Builder::new().point(point).point(point_two).build()?;
        assert_eq!(path.points, vector_test);
        Ok(())
    }

    #[test]
    fn creation_of_path_using_vector_point() -> Result<()> {
        let point = point::Builder::new().x(1.42).y(2.0).build()?;

        let point_two = point::Builder::new()
            .adress("144 rue du bosquet 34980 Saint Clement de riviere".to_string())
            .build()?;

        let mut vector_test = Vec::new();
        vector_test.push(point);
        vector_test.push(point_two);
        let path = Builder::new().points(vector_test.clone()).build()?;
        assert_eq!(path.points, vector_test);
        Ok(())
    }
}
