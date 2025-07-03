/* CSE 381 - Convex Hull (Graham Scan)
*  (c) BYU-Idaho - It is an honor code violation to post this
*  file completed in a public file sharing site. F5.
*
*  Instructions: Refer to W11 Prove: Assignment in Canvas for detailed instructions.
*/

use std::cmp::Ordering;

/* Used for comparing f64 in point objects */
const TOLERANCE: f64 = 0.001;

/* Valid values for angle orientation */
#[derive(Debug, PartialEq)]
pub enum Angle {
    Convex,
    Concave,
    Colinear,
}

/* Structure of a point */
#[derive(Debug, Clone)]
pub struct Point {
    pub x : f64,
    pub y : f64,
}

/* Provide a way to compare if two points are equal.  This is done
   since we can't compare f64's.  This is used in the tests. */
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= TOLERANCE && 
        (self.y - other.y).abs() <= TOLERANCE 
    }
}

/* Determine if 3 points form a convex, concave, or
*  co-linear angle.
*
*  Inputs:
*     a - Point 1
*     b - Point 2
*     c - Point 3
*  Outputs:
*     Return Angle::Convex, Angle::Concave, or Angle::Colinear
*/
fn orientation(a : &Point, b : &Point, c : &Point) -> Angle {
    todo!()
}

/* Determine the angle of a point relative to an anchor point.
*
*  Inputs:
*     anchor - Anchor point
*     point - Other point
*  Outputs:
*     Return angle in radians
*/
fn get_angle(a : &Point, b : &Point) -> f64 {
    todo!()
}

/* Determine the distance from an anchor point to another point
*
*  Inputs:
*     anchor - Anchor point
*     point - Other point
*  Outputs:
*     Return distance
*/
fn get_dist(a : &Point, b : &Point) -> f64 {
    todo!()
}

/* Generate a Convex Hull from a list of points.
*
*  Inputs:
*     points - List of points to create hull around
*  Outputs:
*    An Option containing a list of points representing the hull: Some(hull)
*    The hull should begin and end with the same point.
*
*  Errors: If no hull exists, then return the error Option: None
*/
pub fn gen_hull(points : &[Point]) -> Option<Vec<Point>> {
    todo!()
}

/* Do not modify these tests.
*  To run the tests, use the cargo tool from the command line:
*     Run All Tests: cargo test convex_hull
*     Run One Test:  cargo test convex_hull::tests::<test function name>
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_orientation_convex() {
        let a = Point {x:0.0, y:0.0};
        let b = Point {x:4.0, y:0.0};
        let c = Point {x:3.0, y:1.0};
        let result = orientation(&a, &b, &c);
        assert_eq!(result, Angle::Convex)
    }

    #[test]
    fn test2_orientation_concave() {
        let a = Point {x:4.0, y:0.0};
        let b = Point {x:3.0, y:1.0};
        let c = Point {x:8.0, y:8.0};
        let result = orientation(&a, &b, &c);
        assert_eq!(result, Angle::Concave)
    }

    #[test]
    fn test3_orientation_colinear() {
        let a = Point {x:2.0, y:1.0};
        let b = Point {x:4.0, y:2.0};
        let c = Point {x:6.0, y:3.0};
        let result = orientation(&a, &b, &c);
        assert_eq!(result, Angle::Colinear)
    }

    #[test]
    fn test4_get_angle() {
        let a = Point {x:1.0, y:2.0};
        let b = Point {x:4.0, y:7.0};
        let result = get_angle(&a, &b);
        assert!((result - 1.030).abs() <= TOLERANCE)
    }

    #[test]
    fn test5_get_dist() {
        let a = Point {x:1.0, y:2.0};
        let b = Point {x:4.0, y:7.0};
        let result = get_dist(&a, &b);
        assert!((result - 5.831).abs() <= TOLERANCE)
    }

    #[test]
    fn test6_gen_hull() {
        let points = vec![
            Point {x:0.0, y:0.0},
            Point {x:4.0, y:0.0},
            Point {x:3.0, y:1.0},
            Point {x:1.0, y:1.0},
            Point {x:8.0, y:8.0},
            Point {x:3.0, y:6.0},
            Point {x:1.0, y:4.0},
            Point {x:1.0, y:3.0},
            Point {x:0.0, y:4.0},
            Point {x:0.0, y:2.0},
            Point {x:5.5, y:7.0},
        ];
        let hull = gen_hull(&points);
        assert!(hull.is_some());
        assert_eq!(hull.unwrap(), vec![
            Point {x:0.0, y:0.0},
            Point {x:4.0, y:0.0},
            Point {x:8.0, y:8.0},
            Point {x:3.0, y:6.0},
            Point {x:0.0, y:4.0},
            Point {x:0.0, y:0.0},
        ]);
    }

    #[test]
    fn test7_gen_hull_all_colinear() {
        let points = vec![
            Point {x:2.0, y:1.0},
            Point {x:4.0, y:2.0},
            Point {x:6.0, y:3.0},
            Point {x:8.0, y:4.0},
            Point {x:10.0, y:5.0},
        ];
        let hull = gen_hull(&points);
        assert!(hull.is_none());
    }

    
    #[test]
    fn test8_gen_hull_almost_colinear() {
        let points = vec![
            Point {x:2.0, y:1.0},
            Point {x:4.0, y:2.0},
            Point {x:6.0, y:3.0},
            Point {x:8.0, y:4.0},
            Point {x:10.0, y:6.0},
        ];
        let hull = gen_hull(&points);
        assert!(hull.is_some());
        assert_eq!(hull.unwrap(), vec![
            Point {x:2.0, y:1.0},
            Point {x:8.0, y:4.0},
            Point {x:10.0, y:6.0},
            Point {x:2.0, y:1.0},
        ]);
    }

    #[test]
    fn test9_gen_hull_too_small() {
        let points = vec![
            Point {x:0.0, y:0.0},
            Point {x:1.0, y:1.0},
        ];
        let hull = gen_hull(&points);
        assert!(hull.is_none());
    }
}