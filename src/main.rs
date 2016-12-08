extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate piston_window;
extern crate opengl_graphics;
extern crate nalgebra as na;
extern crate num_traits;

use piston::window::WindowSettings;
use piston::event_loop::*;
use glutin_window::GlutinWindow;
use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL};
use std::path::Path;
use opengl_graphics::glyph_cache::GlyphCache;

use na::{Vector2, Point2};

mod poolball;
mod goalzone;
mod physics;
mod blackhole;
mod math;
mod game;
mod arrow;

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

    // Create some starting poolballs
    let mut balls = vec![
        poolball::Poolball::new(Point2::new(0.5, 0.3), poolball::BallType::White),
        poolball::Poolball::new(Point2::new(0.1, 0.1), poolball::BallType::Red),
        poolball::Poolball::new(Point2::new(0.2, 0.1), poolball::BallType::Red),
        poolball::Poolball::new(Point2::new(0.3, 0.1), poolball::BallType::Red),
        poolball::Poolball::new(Point2::new(0.4, 0.1), poolball::BallType::Red),
        poolball::Poolball::new(Point2::new(0.5, 0.1), poolball::BallType::Red),
        poolball::Poolball::new(Point2::new(0.6, 0.1), poolball::BallType::Red),
        poolball::Poolball::new(Point2::new(0.7, 0.1), poolball::BallType::Red),
        poolball::Poolball::new(Point2::new(0.8, 0.1), poolball::BallType::Red),
        poolball::Poolball::new(Point2::new(0.9, 0.1), poolball::BallType::Red),
        poolball::Poolball::new(Point2::new(0.1, 0.2), poolball::BallType::Red),
        poolball::Poolball::new(Point2::new(0.1, 0.3), poolball::BallType::Red),
        poolball::Poolball::new(Point2::new(0.1, 0.4), poolball::BallType::Red),
        poolball::Poolball::new(Point2::new(0.1, 0.5), poolball::BallType::Red),
        poolball::Poolball::new(Point2::new(0.1, 0.6), poolball::BallType::Red),
        poolball::Poolball::new(Point2::new(0.1, 0.7), poolball::BallType::Blue),
        poolball::Poolball::new(Point2::new(0.1, 0.8), poolball::BallType::Blue),
        poolball::Poolball::new(Point2::new(0.1, 0.9), poolball::BallType::Blue),
    ];

    // Create blackholes
    let blackholes = vec![
        blackhole::Blackhole::new(Point2::new(0.6, 0.7), 0.01, 0.0000001, 0.1),
    ];

    // Create goalzones
    let goalzones = vec![
        goalzone::Goalzone::new(Point2::new(0.0, 0.0)),
        goalzone::Goalzone::new(Point2::new(1.0, 0.0)),
        goalzone::Goalzone::new(Point2::new(0.0, 1.0)),
        goalzone::Goalzone::new(Point2::new(1.0, 1.0)),
    ];

    // Set velocity for all poolballs
    for ball in &mut balls {
        ball.set_velocity(Vector2::new(0.2, -0.3));
    }

    // Create and start the game
    let mut game = game::Game::new(balls, blackholes, goalzones);

    let font_path = Path::new("assets/FiraSans-Regular.ttf");
    let ref mut cache = GlyphCache::new(font_path).unwrap();

    // Main game loop
    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&mut gl, &r, cache);
        }

        if let Some(u) = e.update_args() {
            game.update(&u);
        }

        // Listen for user input
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Space {
                game.try_switch_mode();
            }
        }
    }
}
