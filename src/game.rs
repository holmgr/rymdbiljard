use piston::input::*;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use na::Vector2;
use std::cmp::Ordering;
use std::f64;

use poolball;
use goalzone;
use blackhole;
use physics;

// Struct used for holding information about a ball-ball collision or a
// ball-wall collision (index_B being None).
// Index_a being None signifies that it is the cueball
struct CollisionPair {
    index_a: Option<usize>,
    index_b: Option<usize>,
    time: f64,
}

impl PartialOrd for CollisionPair {
    fn partial_cmp(&self, other: &CollisionPair) -> Option<Ordering> {
        self.time.partial_cmp(&other.time)
    }
}

impl PartialEq for CollisionPair {
    fn eq(&self, other: &CollisionPair) -> bool {
        self.time == other.time
    }
}

pub struct Game {
    cueball: poolball::Poolball,
    balls: Vec<poolball::Poolball>,
    blackholes: Vec<blackhole::Blackhole>,
    goalzones: Vec<goalzone::Goalzone>,
    score: i32,
}

impl Game {
    pub fn new(cueball: poolball::Poolball,
               balls: Vec<poolball::Poolball>,
               blackholes: Vec<blackhole::Blackhole>,
               goalzones: Vec<goalzone::Goalzone>)
               -> Self {
        Game {
            cueball: cueball,
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

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        // Draw score
        gl.draw(args.viewport(), |c, gl| {

            let trans = c.transform
                .trans(0.46 * (args.width as f64), 0.02 * (args.height as f64));
            let score_str = format!("Score: {}", self.score);
            text::Text::new_color(WHITE, 18)
                .draw(score_str.as_str(), cache, &c.draw_state, trans, gl);
        });

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 0.4];

        // Draw goalzones
        let green_ellipse = Ellipse::new(GREEN);
        for goalzone in &self.goalzones {
            goalzone.render(green_ellipse, args, gl);
        }

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let red_ellipse = Ellipse::new(RED);
        let white_ellipse = Ellipse::new(WHITE);

        // Draw cue ball
        self.cueball.render(white_ellipse, args, gl);

        // Draw all other balls
        for ball in &self.balls {
            ball.render(red_ellipse, args, gl);
        }
    }

    // Updates the positon, speeds etc for all cueballs aswell as handling the
    // collisions
    pub fn update(&mut self, args: &UpdateArgs) {

        let mut time_left = args.dt;
        loop {
            let first = self.get_first_collision_pair();
            if first.time <= time_left {
                self.cueball.update(first.time);

                for ball in &mut self.balls {
                    ball.update(first.time);
                }

                time_left -= first.time;

                // Clone the first collision cueball
                let mut ball_a = self.cueball.clone();
                match first.index_a {
                    Some(index) => {
                        ball_a = self.balls.get_mut(index).unwrap().clone();
                    }
                    None => {}
                }

                // Check whether it is a ball-ball or ball-wall collision
                match first.index_b {
                    Some(index) => {
                        let mut ball_b = self.balls.get_mut(index).unwrap().clone();
                        physics::ball_ball_collision(&mut ball_a, &mut ball_b);

                        // Update the second cueball in the pair
                        self.balls[index] = ball_b;
                    }
                    None => {
                        physics::ball_wall_collision(&mut ball_a);
                    }
                }

                // Update the first cueball in the pair
                match first.index_a {
                    Some(index) => {
                        self.balls[index] = ball_a;
                    }
                    None => self.cueball = ball_a,
                }

            } else {
                break;
            }
        }

        // Move the final stretch of time
        self.cueball.update(time_left);

        for ball in &mut self.balls {
            ball.update(time_left);
        }
    }

    // Returns a collision pair for the earlies collison
    fn get_first_collision_pair(&self) -> CollisionPair {

        let ball = &self.cueball;
        let time_wall = physics::time_to_wall_collision(&ball);

        let mut earliest_collision_pair = CollisionPair {
            index_a: None,
            index_b: None,
            time: time_wall,
        };

        for i in 0..self.balls.len() {
            let second = &self.balls[i];

            let time_ball = physics::check_collision(ball, second);
            if time_ball < earliest_collision_pair.time {
                earliest_collision_pair = CollisionPair {
                    index_a: None,
                    index_b: Some(i),
                    time: time_ball,
                }
            }
        }

        for i in 0..self.balls.len() {
            let ball = &self.balls[i];

            let time_wall = physics::time_to_wall_collision(&ball);

            if time_wall < earliest_collision_pair.time {
                earliest_collision_pair = CollisionPair {
                    index_a: Some(i),
                    index_b: None,
                    time: time_wall,
                }
            }

            for j in i + 1..self.balls.len() {
                let second = &self.balls[j];

                let time_ball = physics::check_collision(ball, second);
                if time_ball < earliest_collision_pair.time {
                    earliest_collision_pair = CollisionPair {
                        index_a: Some(i),
                        index_b: Some(j),
                        time: time_ball,
                    }
                }
            }

        }
        earliest_collision_pair
    }
}
