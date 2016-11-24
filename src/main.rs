extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate piston_window;
extern crate opengl_graphics;
extern crate nalgebra as na;
extern crate num_traits;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::Transformed;
use std::path::Path;
use opengl_graphics::glyph_cache::GlyphCache;

use na::{Point2, Vector2};

mod poolball;
mod goalzone;
mod physics;
mod blackhole;


pub struct Game {
    cueball: poolball::Poolball,
    balls: Vec<poolball::Poolball>,
    blackholes: Vec<blackhole::Blackhole>,
    goalzones: Vec<goalzone::Goalzone>,
    score: i32,
}

const BALL_SIZE: f64 = 10.0;

impl Game {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, cache: &mut GlyphCache) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        gl.draw(args.viewport(), |c, g| {
            // Clear the screen.
            clear(BLACK, g);
        });

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        // Draw scoe
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

    fn update(&mut self, gl: &mut GlGraphics, args: &UpdateArgs) {
        self.cueball.update(Vector2::new(0.0, 0.0), args.dt);

        for ball in &mut self.balls {
            ball.update(Vector2::new(0.0, 0.0), args.dt);
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: GlutinWindow = WindowSettings::new("spinning-square", [800, 800])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(opengl);

    let balls = vec![
        poolball::Poolball::new(Point2::new(0.1, 0.1)),
        poolball::Poolball::new(Point2::new(0.3, 0.4)),
        poolball::Poolball::new(Point2::new(0.7, 0.5)),
        poolball::Poolball::new(Point2::new(0.6, 0.6)),
    ];

    let goalzones = vec![
        goalzone::Goalzone::new(Point2::new(0.0, 0.0)),
        goalzone::Goalzone::new(Point2::new(1.0, 0.0)),
        goalzone::Goalzone::new(Point2::new(0.0, 1.0)),
        goalzone::Goalzone::new(Point2::new(1.0, 1.0)),
    ];

    let mut game = Game {
        cueball: poolball::Poolball::new(Point2::new(0.5, 0.9)),
        balls: balls,
        blackholes: Vec::new(),
        goalzones: goalzones,
        score: 0,
    };

    game.cueball.set_velocity(&Vector2::new(0.01, -0.01));

    let font_path = Path::new("assets/FiraSans-Regular.ttf");
    let ref mut cache = GlyphCache::new(font_path).unwrap();

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&mut gl, &r, cache);
        }

        if let Some(u) = e.update_args() {
            game.update(&mut gl, &u);
        }
    }
}
