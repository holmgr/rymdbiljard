use na::{Vector2, Point2};

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
}

#[test]
fn test_update() {
    let mut ball = Poolball::new(Point2::new(0.0, 0.0));
    ball.update(Vector2::new(1.0, 1.0), 1.0);
    ball.update(Vector2::new(1.0, 1.0), 1.0);
    assert_eq!(ball.velocity, Vector2::new(2.0, 2.0));
    assert_eq!(ball.position, Point2::new(1.0, 1.0));
}
