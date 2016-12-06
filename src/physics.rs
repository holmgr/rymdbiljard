use na::{Vector2, Point2, Norm, FloatPoint, Dot};
use poolball;
use blackhole;
use math;
use std::f64;

const FRICTION: f64 = 0.01;

/**
 * Calculates the gravity acceleration from an object with given mass at given
 * distance
 */
fn gravity_acc(mass: f64, distance: f64) -> f64 {
    // let gravity_constant = 6.673 * (10.0_f32).powf(-11.0);
    // We use gravity_constant = 1 for now, as the actual constant would require very high mass
    let gravity_constant = 1.0;
    (gravity_constant * mass) / distance.powf(2.0)
}

/**
 * Calculates the acceleration acting upon the ball from the given black holes
 */
pub fn calculate_gravity(blackholes: &Vec<blackhole::Blackhole>,
                         ball: &poolball::Poolball)
                         -> Vector2<f64> {
    let mut result = Vector2::new(0.0, 0.0);
    // Calculate each acceleration vector individually and add them to the reuslt
    for blackhole in blackholes {
        let distance = ball.position.distance(&blackhole.position);

        if distance < blackhole.reach {
            let direction_vector = blackhole.position.to_vector() - ball.position.to_vector();
            let normalized_vector = direction_vector.normalize();
            result += normalized_vector * gravity_acc(blackhole.mass, distance);
        }
    }
    return result;
}

/**
 * Calculates the direction and size of the friction acceleration on the given
 * ball
 */
pub fn friction(poolball: &poolball::Poolball) -> Vector2<f64> {
    if poolball.velocity == Vector2::new(0.0, 0.0) {
        return Vector2::new(0.0, 0.0);
    }
    let ball_direction = poolball.velocity.normalize();
    let friction = -1.0 * ball_direction * FRICTION;
    return friction;
}

/**
* Calculates the new velocities for 2 colloding bals after time delta_time,
* (delta_time is in seconds)
*/
pub fn ball_ball_collision(ball1: &mut poolball::Poolball, ball2: &mut poolball::Poolball) {

    // Find the Normal for the 2 balls
    let mut n: Vector2<f64> = ball1.position - ball2.position;
    n = n.normalize();

    // find the lengths of the component of each of the movements
    let a1 = ball1.velocity.dot(&n);
    let a2 = ball2.velocity.dot(&n);

    // Calculate a common component in the formula
    // optimizedP
    let optimized_p: f64 = (2.0 * (a1 - a2)) / (ball1.mass + ball2.mass);

    // Calculate the new movementvector for the balls
    let new_v1 = ball1.velocity - optimized_p * ball1.mass * n;
    let new_v2 = ball2.velocity + optimized_p * ball2.mass * n;

    // Set the new velocities for the balls
    ball1.velocity = new_v1;
    ball2.velocity = new_v2;
}

/**
 * Checks for collision between the given balls
 */
pub fn time_to_ball_ball_collision(a: &poolball::Poolball, b: &poolball::Poolball) -> f64 {
    // We pretend that b is stationary and compensate by subtracting its movement vector from a's
    let move_vec = a.velocity - b.velocity;

    // Calculate the distance between the centers and their combined radius
    // let mut dist = a.position.distance(&b.position);
    let sum_radii = a.radius + b.radius;

    // If we are not moving a large enough distance, we will not collide
    // dist-= sum_radii;

    // Normalize the movement vector
    let normalized_vector = move_vec.normalize();

    // Calculate the direction between the two balls
    let direction_between = (b.position - a.position) as Vector2<f64>;

    // Calculate the dot product of the normalied vector and the direction
    let dot_product = Vector2::dot(&direction_between, &normalized_vector);

    // If the dot product is 0 or negative, we are moving in the opposite direction
    if dot_product <= 0.0 {
        return f64::INFINITY;
    }

    // Find the length of the direction vector between them
    let length_c = math::calc_magnitude(direction_between);

    // If the closest that A will get to B is larger than their
    // combined radii, there will be no collision
    let f = (length_c * length_c) - (dot_product * dot_product);
    let sum_radii_squared = sum_radii * sum_radii;
    if f > sum_radii_squared {
        return f64::INFINITY;
    }
    let t = sum_radii_squared - f;

    if t < 0.0 {
        return f64::INFINITY;
    }

    let movement_distance = dot_product - t.sqrt();
    let move_vec_magnitude = math::calc_magnitude(move_vec);

    return movement_distance / move_vec_magnitude;
}

/**
 * Return the time to impact with wall given the current velocity
 */
pub fn time_to_wall_collision(ball: &poolball::Poolball) -> f64 {
    // will be the distance to the wall in the x direction the ball is moving
    let horizontal_distance_to_wall = (ball.position.x - (ball.velocity.x.signum() / 2.0 + 0.5))
        .abs() - ball.radius;
    // will be the distance to the wall in the y direction the ball is moving
    let vertical_distance_to_wall = (ball.position.y - (ball.velocity.y.signum() / 2.0 + 0.5))
        .abs() - ball.radius;

    let x_time_ratio = horizontal_distance_to_wall / ball.velocity.x.abs();
    let y_time_ratio = vertical_distance_to_wall / ball.velocity.y.abs();

    let min_time = x_time_ratio.min(y_time_ratio);
    return min_time;
}

/**
 * Recalculates the new velocities for the ball given collision with a wall
 */
pub fn ball_wall_collision(ball: &mut poolball::Poolball) {
    // will be the distance to the wall in the x direction the ball is moving
    let horizontal_distance_to_wall =
        ((ball.position.x - (ball.velocity.x.signum() / 2.0 + 0.5)).abs() - ball.radius).abs();
    // will be the distance to the wall in the y direction the ball is moving
    let vertical_distance_to_wall =
        ((ball.position.y - (ball.velocity.y.signum() / 2.0 + 0.5)).abs() - ball.radius).abs();

    // change the velocity given which wall was hit (the closest)
    let mut tmp = ball.velocity;
    if horizontal_distance_to_wall < vertical_distance_to_wall {
        tmp.x = tmp.x * (-1.0);
    } else {
        tmp.y = tmp.y * (-1.0);
    }
    ball.set_velocity(tmp);
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
    let blackholes = vec![blackhole::Blackhole::new(Point2::new(0.0, 0.0), 1.0, 1.0, 1.0),
                          blackhole::Blackhole::new(Point2::new(0.0, 1.0), 1.0, 1.0, 1.0)];
    let ball = poolball::Poolball::new(Point2::new(1.0, 1.0), poolball::BallType::Red);
    let acc_vector = calculate_gravity(&blackholes, &ball);
    assert!((acc_vector.len() as f64) -
            ((1.0 / (8.0_f64).sqrt() *
              Vector2::new(-1.0 - (8.0_f64).sqrt(),
                           -1.0))
        .len() as f64) < 0.001);
}

// Testing that planets have no effect on the ball if it is out of their reach
#[test]
fn test_calculate_gravity_reach() {
    let blackholes = vec![blackhole::Blackhole::new(Point2::new(0.0, 0.0), 1.0, 1.0, 0.0),
                          blackhole::Blackhole::new(Point2::new(0.0, 1.0), 1.0, 1.0, 0.0)];
    let ball = poolball::Poolball::new(Point2::new(1.0, 1.0), poolball::BallType::Red);
    let acc_vector = calculate_gravity(&blackholes, &ball);
    assert_eq!(acc_vector, Vector2::new(0.0, 0.0));
}

#[test]
fn test_friction() {
    let mut ball = poolball::Poolball::new(Point2::new(1.0, 1.0), poolball::BallType::Red);
    assert_eq!(friction(&mut ball), Vector2::new(0.0, 0.0));

    let mut ball2 = poolball::Poolball::new(Point2::new(1.0, 1.0), poolball::BallType::Red);
    ball2.velocity = Vector2::new(1.0, 0.0);
    assert_eq!(Vector2::new(-1.0, 0.0) * FRICTION, friction(&mut ball2));
}


#[test]
fn test_check_collision_simple() {
    let mut ball1 = poolball::Poolball::new(Point2::new(0.0, 0.0), poolball::BallType::Red);
    ball1.radius = 1.0;
    let mut ball2 = poolball::Poolball::new(Point2::new(3.0, 0.0), poolball::BallType::Red);
    ball2.radius = 1.0;

    ball1.velocity = Vector2::new(1.0, 0.0);
    let collision_time = time_to_ball_ball_collision(&ball1, &ball2);

    assert_eq!(collision_time, 1.0);

    let mut ball1 = poolball::Poolball::new(Point2::new(0.0, 0.0), poolball::BallType::Red);
    ball1.radius = 1.0;
    let mut ball2 = poolball::Poolball::new(Point2::new(4.0, 0.0), poolball::BallType::Red);
    ball2.radius = 1.0;

    ball1.velocity = Vector2::new(1.0, 0.0);

    let collision_time = time_to_ball_ball_collision(&ball1, &ball2);

    assert_eq!(collision_time, 2.0);
}

#[test]
fn test_check_collision_advanced() {
    let mut ball1 = poolball::Poolball::new(Point2::new(0.0, 0.0), poolball::BallType::Red);
    ball1.radius = 1.0;
    let mut ball2 = poolball::Poolball::new(Point2::new(4.0, 0.0), poolball::BallType::Red);
    ball2.radius = 1.0;

    ball1.velocity = Vector2::new(1.0, 1.0);
    ball2.velocity = Vector2::new(-1.0, 1.0);

    ball1.set_velocity(Vector2::new(1.0, 1.0));
    ball2.set_velocity(Vector2::new(-1.0, 1.0));

    let collision_time = time_to_ball_ball_collision(&ball1, &ball2);

    assert_eq!(collision_time, 1.0);
}

#[test]
fn test_simpl_ball_ball_collision_calculation() {
    let mut ball1 = poolball::Poolball::new(Point2::new(0.0, 0.0), poolball::BallType::Red);
    ball1.mass = 1.0;
    ball1.velocity = Vector2::new(1.0, 0.0);

    let mut ball2 = poolball::Poolball::new(Point2::new(4.0, 0.0), poolball::BallType::Red);
    ball2.mass = 1.0;
    ball2.velocity = Vector2::new(-1.0, 0.0);

    ball_ball_collision(&mut ball1, &mut ball2);
    assert_eq!(ball1.velocity, Vector2::new(-1.0, 0.0));
    assert_eq!(ball2.velocity, Vector2::new(1.0, 0.0));
}

#[test]
fn test_diagonal_ball_ball_collision_calculation() {
    let mut ball1 = poolball::Poolball::new(Point2::new(0.0, 0.0), poolball::BallType::Red);
    ball1.mass = 1.0;
    ball1.velocity = Vector2::new(1.0, 1.0);

    let mut ball2 = poolball::Poolball::new(Point2::new(4.0, 0.0), poolball::BallType::Red);
    ball2.mass = 1.0;
    ball2.velocity = Vector2::new(-1.0, 1.0);

    ball_ball_collision(&mut ball1, &mut ball2);
    assert_eq!(ball1.velocity, Vector2::new(-1.0, 1.0));
    assert_eq!(ball2.velocity, Vector2::new(1.0, 1.0));
}

#[test]
fn test_time_to_wall_collision() {
    let mut ball = poolball::Poolball::new(Point2::new(0.4, 0.5), poolball::BallType::Red);
    ball.radius = 0.1;
    ball.velocity = Vector2::new(1.0, 0.0);
    assert_eq!(time_to_wall_collision(&ball), 0.5);
    ball.position = Point2::new(0.6, 0.5);
    ball.velocity = Vector2::new(-1.0, 0.0);
    assert_eq!(time_to_wall_collision(&ball), 0.5);
    ball.position = Point2::new(0.5, 0.4);
    ball.velocity = Vector2::new(0.0, 1.0);
    assert_eq!(time_to_wall_collision(&ball), 0.5);
    ball.position = Point2::new(0.5, 0.6);
    ball.velocity = Vector2::new(0.0, -1.0);
    assert_eq!(time_to_wall_collision(&ball), 0.5);
    ball.position = Point2::new(0.4, 0.4);
    ball.velocity = Vector2::new(1.0, 1.0);
    assert_eq!(time_to_wall_collision(&ball), 0.5);
    ball.position = Point2::new(0.6, 0.5);
    ball.velocity = Vector2::new(-2.0, 0.0);
    assert_eq!(time_to_wall_collision(&ball), 0.25)
}

#[test]
fn test_ball_wall_collision() {
    let mut ball = poolball::Poolball::new(Point2::new(0.9, 0.5), poolball::BallType::Red);
    ball.radius = 0.1;
    ball.velocity = Vector2::new(1.0, 0.0);
    ball_wall_collision(&mut ball);
    assert_eq!(ball.velocity, Vector2::new(-1.0, 0.0));
    ball.position = Point2::new(0.5, 0.9);
    ball.velocity = Vector2::new(0.0, 1.0);
    ball_wall_collision(&mut ball);
    assert_eq!(ball.velocity, Vector2::new(0.0, -1.0));
    ball.position = Point2::new(0.5, 0.1);
    ball.velocity = Vector2::new(0.0, -1.0);
    ball_wall_collision(&mut ball);
    assert_eq!(ball.velocity, Vector2::new(0.0, 1.0));
    ball.position = Point2::new(0.1, 0.9);
    ball.velocity = Vector2::new(-1.0, 0.0);
    ball_wall_collision(&mut ball);
    assert_eq!(ball.velocity, Vector2::new(1.0, 0.0));
}
