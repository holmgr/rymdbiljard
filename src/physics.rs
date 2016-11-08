use std::f32;
use na::{Vector2, Point2, Norm, FloatPoint};
use golfball;
use planet;

// Calculates the gravity force from an object with given mass at given distance
fn gravity_acc(mass: f32, distance: f32) -> f32 {
    // let gravity_constant = 6.673 * (10.0_f32).powf(-11.0);
    // We use gravity_constant = 1 for now, as the actual constant would require very high mass
    let gravity_constant = 1.0;
    (gravity_constant * mass) / distance.powf(2.0)
}

// Calculates the acceleration acting upon the ball from the given planets
pub fn calculate_gravity(planets: Vec<planet::Planet>, ball: golfball::Golfball) -> Vector2<f32> {
    let mut result = Vector2::new(0.0, 0.0);
    // Calculate each acceleration vector individually and add them to the reuslt
    for planet in &planets {
        let distance = ball.position.distance(&planet.position);

        if distance < planet.reach {
            let direction_vector = planet.position.to_vector() - ball.position.to_vector();
            let normalized_vector = direction_vector.normalize();
            result += normalized_vector * gravity_acc(planet.mass, distance);
        }

    }
    return result;
}

// Basic tests for gravity_acc
#[test]
fn test_gravity_acc() {
    let acceleration = gravity_acc(1.0, 1.0);
    assert_eq!(acceleration, 1.0);
    let acceleration = gravity_acc(1.0, 2.0);
    assert_eq!(acceleration, 0.25);
    let acceleration = gravity_acc(2.5, 2.0);
    assert_eq!(acceleration, 0.625);
}


// Basic test for gravity_acc. Has some error-margin because of rounding errors.
#[test]
fn test_calculate_gravity() {
    let planets = vec![planet::Planet {
                           position: Point2::new(0.0, 0.0),
                           mass: 1.0,
                           radius: 1.0,
                           reach: 1.0,
                       },
                       planet::Planet {
                           position: Point2::new(0.0, 1.0),
                           mass: 1.0,
                           radius: 1.0,
                           reach: 1.0,
                       }];
    let ball = golfball::Golfball::new(Point2::new(1.0, 1.0));
    let acc_vector = calculate_gravity(planets, ball);
    assert!((acc_vector.len() as f32) -
            ((1.0 / (8.0_f32).sqrt() *
              Vector2::new(-1.0 - (8.0_f32).sqrt(),
                           -1.0))
        .len() as f32) < 0.001);
}

// Testing that planets have no effect on the ball if it is out of their reach
#[test]
fn test_calculate_gravity_reach() {
    let planets = vec![planet::Planet {
                           position: Point2::new(0.0, 0.0),
                           mass: 1.0,
                           radius: 1.0,
                           reach: 0.0,
                       },
                       planet::Planet {
                           position: Point2::new(0.0, 1.0),
                           mass: 1.0,
                           radius: 1.0,
                           reach: 0.0,
                       }];
    let ball = golfball::Golfball::new(Point2::new(1.0, 1.0));
    let acc_vector = calculate_gravity(planets, ball);
    assert_eq!(acc_vector, Vector2::new(0.0, 0.0));
}