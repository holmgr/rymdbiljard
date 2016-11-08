use na::{Point2, FloatPoint};
use poolball;

// Implements a basic goalzone to be used in each corner of the playing field.
pub struct Goalzone {
    position: Point2<f64>,
    radius: f64,
}
const RADIUS: f64 = 10.0;

impl Goalzone {
    // Creates a new goalzone at the given position
    pub fn new(position: Point2<f64>) -> Goalzone {
        Goalzone {
            position: position,
            radius: RADIUS,
        }
    }

    // Returns whether a given poolball is within the goalzone
    pub fn reached_goal(&self, ball: &poolball::Poolball) -> bool {
        self.position.distance(&ball.position) <= self.radius + ball.radius
    }
}

#[test]
fn test_reach() {
    // Inside range, should succeed
    let ball = poolball::Poolball::new(Point2::new(0.0, 0.0));
    let goalzone = Goalzone::new(Point2::new(0.0, 10.0));
    assert!(goalzone.reached_goal(&ball));

    // Outside of range should fail
    let goalzone = Goalzone::new(Point2::new(10.0, 10.0));
    assert!(!goalzone.reached_goal(&ball));
}
