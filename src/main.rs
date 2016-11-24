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
mod game;

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

    let mut cueball = poolball::Poolball::new(Point2::new(0.5, 0.9));
    cueball.set_velocity(&Vector2::new(0.01, -0.01));

    let mut game = game::Game::new(
        cueball,
        balls,
        Vec::new(),
        goalzones,
    );


    let font_path = Path::new("assets/FiraSans-Regular.ttf");
    let ref mut cache = GlyphCache::new(font_path).unwrap();

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&mut gl, &r, cache);
        }

        if let Some(u) = e.update_args() {
            game.update(&u);
        }
    }
}
