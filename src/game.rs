use piston::input::*;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use na::Vector2;

use poolball;
use goalzone;
use blackhole;

pub struct Game {
    cueball: poolball::Poolball,
    balls: Vec<poolball::Poolball>,
    blackholes: Vec<blackhole::Blackhole>,
    goalzones: Vec<goalzone::Goalzone>,
    score: i32,
}

impl Game {
    pub fn new(cueball: poolball::Poolball, balls: Vec<poolball::Poolball>, blackholes: Vec<blackhole::Blackhole>, goalzones: Vec<goalzone::Goalzone>) -> Self {
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
        gl.draw(args.viewport(), |c, g| {
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

    pub fn update(&mut self, args: &UpdateArgs) {
        self.cueball.update(Vector2::new(0.0, 0.0), args.dt);

        for ball in &mut self.balls {
            ball.update(Vector2::new(0.0, 0.0), args.dt);
        }
    }
}
