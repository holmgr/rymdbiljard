use piston::input::*;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use na::Point2;
use std::f64;
use std::process::exit;

use poolball;
use goalzone;
use blackhole;
use physics;

/**
 * Struct used for holding information about a ball-ball collision or a
 * ball-wall collision.
 * A ball-wall collision is signified by Second being none.
 */
struct CollisionPair {
    first: poolball::Poolball,
    second: Option<poolball::Poolball>,
    time: f64,
}

/**
 * Main struct for the global game state information, such as the cueballs in
 * play, goalzones and eventual blackholes etc.
 * Also implements the main gameplay functionality such as the update function
 * and collision handling algorithm.
 */
pub struct Game {
    balls: Vec<poolball::Poolball>,
    blackholes: Vec<blackhole::Blackhole>,
    goalzones: Vec<goalzone::Goalzone>,
    score: i32,
}

impl Game {
    /**
     * Creates a new game given the specified parameters
     */
    pub fn new(balls: Vec<poolball::Poolball>,
               blackholes: Vec<blackhole::Blackhole>,
               goalzones: Vec<goalzone::Goalzone>)
               -> Self {
        Game {
            balls: balls,
            blackholes: blackholes,
            goalzones: goalzones,
            score: 0,
        }
    }

    /**
     * Renders the current game state including the cueballs, current score,
     * blackholes and goalzones using the GlGraphics
     */
    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, cache: &mut GlyphCache) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        gl.draw(args.viewport(), |_, g| {
            // Clear the screen.
            clear(BLACK, g);
        });

        // Draw score
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        gl.draw(args.viewport(), |c, gl| {

            let trans = c.transform
                .trans(0.46 * (args.width as f64), 0.02 * (args.height as f64));
            let score_str = format!("Score: {}", self.score);
            text::Text::new_color(WHITE, 18)
                .draw(score_str.as_str(), cache, &c.draw_state, trans, gl);
        });

        // Draw goalzones
        for goalzone in &self.goalzones {
            goalzone.render(args, gl);
        }

        // Draw all cueballs
        for ball in &self.balls {
            ball.render(args, gl);
        }

        // Draw all blackholes
        for blackhole in &self.blackholes {
            blackhole.render(args, gl);
        }
    }

    /**
     * Updates the positon, speeds etc for all cueballs aswell as handling the
     * collisions
    */
    pub fn update(&mut self, args: &UpdateArgs) {

        // Save tatal time budget
        let mut time_left = args.dt;
        let CollisionPair { mut first, mut second, mut time } = self.get_first_collision_pair();

        // While there exists a collision within this time step
        while time < time_left {

            // Remove the collision pair form the list of cueballs
            self.balls.retain(|elem| {
                match second {
                    Some(ref mut second) => *elem != first && *elem != *second,
                    None => *elem != first,
                }
            });

            // No collisions can occure before the first one. Move all balls
            // using their current velocities
            for ball in &mut self.balls {
                ball.update(time);
            }

            first.update(time);
            if let Some(ref mut second) = second {
                second.update(time);
            }

            // Reduce time left
            time_left -= time;

            // Solve the collision: either ball-wall or ball-ball
            match second {
                Some(mut second) => {
                    physics::ball_ball_collision(&mut first, &mut second);

                    // Add updated first and second back
                    self.balls.push(first);
                    self.balls.push(second);
                }
                None => {
                    physics::ball_wall_collision(&mut first);

                    // Add updated first and second back
                    self.balls.push(first);
                }
            }

            // Get the next collision pair
            let pair = self.get_first_collision_pair();
            first = pair.first;
            second = pair.second;
            time = pair.time;

        }

        // If there is time left, advance the rest of the time step
        for ball in &mut self.balls {
            ball.update(time_left);
        }

        // Add friction for this time step
        for ball in &mut self.balls {
            let friction = physics::calculate_friction(ball);
            ball.update_velocity(friction, args.dt);
        }

        // Check if white ball exists, spawn new if not as long as the score is
        // positive
        let pos = white_ball_position(&self.balls);
        match pos {
            // White ball is dead but we have enough score to spawn a new one
            None if self.score > 0 => {
                let new_white_ball = poolball::Poolball::new(Point2::new(0.1, 0.1),
                                                             poolball::BallType::White);
                self.balls.push(new_white_ball);
            }
            // No score left to respawn, game over
            None => {
                exit(0); // Game over kills processs as of now
            }
            _ => {}
        }

        // Add accelerations for all balls within blackholes
        let blackholes = &self.blackholes;

        for ball in &mut self.balls {
            let acceleration = physics::calculate_gravity(blackholes, ball);
            ball.update_velocity(acceleration, args.dt)
        }
        let balls = &mut self.balls;

        balls.retain(|ball| !blackholes.iter().any(|hole| hole.is_spagettified(ball)));

        // Check if any balls are in the goalzones, removing and adding score
        // accordingly
        let goalzones = &self.goalzones;
        let mut score = self.score;

        balls.retain(|ball| {
            match goalzones.iter().any(|zone| zone.reached_goal(ball)) {
                true => {
                    score += ball.get_value();
                    false
                }
                false => true,
            }
        });

        self.score = score;
    }

    /**
     * Returns a collision pair for the earlies collision by going throguh all
     * cueballs searching for the ball-wall or ball-ball pair with the earlies
     * collision time
     */
    fn get_first_collision_pair(&self) -> CollisionPair {

        let mut earliest_collision_pair = CollisionPair {
            first: poolball::Poolball::new(Point2::new(0.0, 0.0), poolball::BallType::Red),
            second: None,
            time: f64::INFINITY,
        };

        // Go throguh all cueballs
        let mut iter = self.balls.iter();
        while let Some(first) = iter.next() {

            // Check collision time for ball-wall
            let time_wall = physics::time_to_wall_collision(first);

            if time_wall < earliest_collision_pair.time {
                earliest_collision_pair = CollisionPair {
                    first: first.clone(),
                    second: None,
                    time: time_wall,
                };
            }

            // Go through the rest of the cue balls and check the pairs for
            // the collision time
            for second in iter.clone().by_ref() {
                let time_ball = physics::time_to_ball_ball_collision(first, second);
                if time_ball < earliest_collision_pair.time {
                    earliest_collision_pair = CollisionPair {
                        first: first.clone(),
                        second: Some(second.clone()),
                        time: time_ball,
                    };
                }
            }

        }
        earliest_collision_pair
    }
}

/**
* Searches for the white ball in the vector returning its index
*/
fn white_ball_position(balls: &Vec<poolball::Poolball>) -> Option<usize> {
    balls.iter().position(|elem| elem.ball_type == poolball::BallType::White)
}
