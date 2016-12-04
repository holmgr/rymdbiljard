use piston::input::*;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use na::{Vector2, Point2};
use std::f64;

use poolball;
use goalzone;
use blackhole;
use physics;

// Struct used for holding information about a ball-ball collision or a
// ball-wall collision (index_B being None).
// Index_a being None signifies that it is the cueball
struct CollisionPair {
    first: poolball::Poolball,
    second: Option<poolball::Poolball>,
    time: f64,
}

pub struct Game {
    balls: Vec<poolball::Poolball>,
    blackholes: Vec<blackhole::Blackhole>,
    goalzones: Vec<goalzone::Goalzone>,
    score: i32,
}

impl Game {
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
    }

    // Updates the positon, speeds etc for all cueballs aswell as handling the
    // collisions
    pub fn update(&mut self, args: &UpdateArgs) {

        let mut time_left = args.dt;

        let CollisionPair { mut first, mut second, mut time } = self.get_first_collision_pair();

        while time < time_left {

            // Remove first and second from the list
            self.balls.retain(|elem| {
                match second {
                    Some(ref mut second) => *elem != first && *elem != *second,
                    None => *elem != first,
                }
            });

            // Move until first collision
            for ball in &mut self.balls {
                ball.update(time);
            }

            first.update(time);
            if let Some(ref mut second) = second {
                second.update(time);
            }

            // Reduce time left
            time_left -= time;

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

            let pair = self.get_first_collision_pair();
            first = pair.first;
            second = pair.second;
            time = pair.time;

        }

        for ball in &mut self.balls {
            ball.update(time_left);
        }
    }

    // Returns a collision pair for the earlies collison
    fn get_first_collision_pair(&self) -> CollisionPair {

        let mut earliest_collision_pair = CollisionPair {
            first: poolball::Poolball::new(Point2::new(0.0, 0.0), poolball::BallType::Red),
            second: None,
            time: f64::INFINITY,
        };

        let mut iter = self.balls.iter();
        while let Some(first) = iter.next() {
            let time_wall = physics::time_to_wall_collision(first);

            if time_wall < earliest_collision_pair.time {
                earliest_collision_pair = CollisionPair {
                    first: first.clone(),
                    second: None,
                    time: time_wall,
                };
            }

            for second in iter.clone().by_ref() {
                let time_ball = physics::check_collision(first, second);
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
