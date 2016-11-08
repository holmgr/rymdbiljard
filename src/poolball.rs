use na::{Vector2, Point2};
use num_traits::Zero;

pub struct Poolball {
    pub position: Point2<f64>,
    pub radius: f64,
    pub mass: f64,
    pub velocity: Vector2<f64>,
}

const MASS: f64 = 0.1;
const RADIUS: f64 = 1.0;

impl Poolball {
    // Creates a new Golfball with a initial position and velocity
    pub fn new(position: Point2<f64>) -> Poolball {
        Poolball {
            position: position,
            radius: RADIUS,
            mass: MASS,
            velocity: Vector2::new(0.0, 0.0),
        }
    }

    // Updates the balls position using its current velocity, then updating velocity
    pub fn update(&mut self, acceleration: Vector2<f64>, delta_time: f64) {
        self.position += self.velocity * delta_time;
        self.velocity += acceleration * delta_time;
    }

    // Sets the velocity of the ball to the given velocity
    pub fn set_velocity(&mut self, new_velocity: &Vector2<f64>) {
        self.velocity = new_velocity.clone();
    }

    // Returns true if the poolball is stationary
    pub fn is_stationary(&self) -> bool {
        self.velocity.is_zero()
    }
}

#[test]
fn test_update() {
    let mut ball = Poolball::new(Point2::new(0.0, 0.0));
    ball.update(Vector2::new(1.0, 1.0), 1.0);
    ball.update(Vector2::new(1.0, 1.0), 1.0);
    assert_eq!(ball.velocity, Vector2::new(2.0, 2.0));
    assert_eq!(ball.position, Point2::new(1.0, 1.0));
}

#[test]
fn test_set_velocity() {
    let mut ball = Poolball::new(Point2::new(0.0, 0.0));
    let new_velocity = Vector2::new(1.0, 1.0);
    ball.set_velocity(&new_velocity);
    assert_eq!(ball.velocity, new_velocity);
}

#[test]
fn test_is_stationary() {
    let mut ball = Poolball::new(Point2::new(0.0, 0.0));
    assert!(ball.is_stationary());
    ball.set_velocity(&Vector2::new(1.0, 1.0));
    assert!(!ball.is_stationary());
}
