use na::{Vector2, Point2};

pub struct Golfball {
  position: Point2<f32>,
  radius: f32,
  mass: f32,
  velocity: Vector2<f32>,
}

const MASS: f32 = 0.1;
const RADIUS: f32 = 1.0;

impl Golfball {

  // Creates a new Golfball with a initial position and velocity
  pub fn new(position: Point2<f32>, velocity: Vector2<f32>) -> Golfball {
    Golfball {
      position: position,
      radius: RADIUS,
      mass: MASS,
      velocity: velocity,
    }
  }

  // Updates the balls position using its current velocity, then updating velocity
  pub fn update(&mut self, acceleration: Vector2<f32>, delta_time: f32) {
    self.position += self.velocity * delta_time;
    self.velocity += acceleration * delta_time;
  }
}

#[test]
fn test_update() {
  let mut ball = Golfball::new(Point2::new(0.0,0.0), Vector2::new(1.0,1.0));
  ball.update(Vector2::new(1.0,1.0), 1.0);
  assert_eq!(ball.velocity, Vector2::new(2.0,2.0));
  assert_eq!(ball.position, Point2::new(1.0, 1.0));
}
