use na::{Vector2, Point2, Norm, FloatPoint};
use poolball;
use blackhole;

const FRICTION: f64 = 0.00001;

// Calculates the gravity acceleration from an object with given mass at given distance
fn gravity_acc(mass: f64, distance: f64) -> f64 {
    // let gravity_constant = 6.673 * (10.0_f32).powf(-11.0);
    // We use gravity_constant = 1 for now, as the actual constant would require very high mass
    let gravity_constant = 1.0;
    (gravity_constant * mass) / distance.powf(2.0)
}

// Calculates the acceleration acting upon the ball from the given black holes
pub fn calculate_gravity(blackholes: Vec<blackhole::Blackhole>, ball: poolball::Poolball) -> Vector2<f64> {
    let mut result = Vector2::new(0.0, 0.0);
    // Calculate each acceleration vector individually and add them to the reuslt
    for blackhole in &blackholes {
        let distance = ball.position.distance(&blackhole.position);

        if distance < blackhole.reach {
            let direction_vector = blackhole.position.to_vector() - ball.position.to_vector();
            let normalized_vector = direction_vector.normalize();
            result += normalized_vector * gravity_acc(blackhole.mass, distance);
        }

    }
    return result;
}

//Calculates the direction and size of the friction acceleration on the given ball
pub fn friction(poolball: poolball::Poolball) -> Vector2<f64> {
	if poolball.velocity == Vector2::new(0.0,0.0){
		return Vector2::new(0.0,0.0);
	}
	let ball_direction = poolball.velocity.normalize();
	let friction = -1.0 * ball_direction * FRICTION; 
	return friction;
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
    let blackholes = vec![blackhole::Blackhole::new(
                           Point2::new(0.0, 0.0),
                           1.0,
                           1.0,
                           1.0,
                       ),
                       blackhole::Blackhole::new(
                           Point2::new(0.0, 1.0),
                           1.0,
                           1.0,
                           1.0,
                       )];
    let ball = poolball::Poolball::new(Point2::new(1.0, 1.0));
    let acc_vector = calculate_gravity(blackholes, ball);
    assert!((acc_vector.len() as f64) -
            ((1.0 / (8.0_f64).sqrt() *
              Vector2::new(-1.0 - (8.0_f64).sqrt(),
                           -1.0))
        .len() as f64) < 0.001);
}

// Testing that planets have no effect on the ball if it is out of their reach
#[test]
fn test_calculate_gravity_reach() {
    let blackholes = vec![blackhole::Blackhole::new(
                           Point2::new(0.0, 0.0),
                           1.0,
                           1.0,
                           0.0,
                       ),
                       blackhole::Blackhole::new(
                           Point2::new(0.0, 1.0),
                           1.0,
                           1.0,
                           0.0,
                       )];
    let ball = poolball::Poolball::new(Point2::new(1.0, 1.0));
    let acc_vector = calculate_gravity(blackholes, ball);
    assert_eq!(acc_vector, Vector2::new(0.0, 0.0));
}

#[test]
fn test_friction() {
	let mut ball = poolball::Poolball::new(Point2::new(1.0, 1.0));
	assert_eq!(friction(ball), Vector2::new(0.0,0.0));

	let mut ball2 = poolball::Poolball::new(Point2::new(1.0, 1.0));
	ball2.velocity = Vector2::new(1.0,0.0);
	assert_eq!(Vector2::new(-1.0, 0.0) * FRICTION, friction(ball2));
}
