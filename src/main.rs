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
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston_window::ellipse::circle;
use na::Point2;

mod poolball;
mod goalzone;
mod physics;
mod blackhole;


pub struct Game {
    cueball: poolball::Poolball,
    balls: Vec<poolball::Poolball>,
    blackholes: Vec<blackhole::Blackhole>,
    goalzones: Vec<goalzone::Goalzone>,
}

const BALL_SIZE: f64 = 10.0;

impl Game {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        gl.draw(args.viewport(), |c, g| {
            // Clear the screen.
            clear(BLACK, g);
        });

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
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

    fn update(&mut self, gl: &mut GlGraphics, args: &UpdateArgs) {}
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [800, 800])
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

    let mut game = Game {
        cueball: poolball::Poolball::new(Point2::new(0.5, 0.9)),
        balls: balls,
        blackholes: Vec::new(),
        goalzones: Vec::new(),
    };


    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&mut gl, &r);
        }

        if let Some(u) = e.update_args() {
            game.update(&mut gl, &u);
        }
    }
}
