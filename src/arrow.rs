use piston::input::RenderArgs;
use opengl_graphics::GlGraphics;
use graphics::Transformed;
use graphics::line;
use na::{Norm, Point2, Vector1, Vector2, Rotation2, Rotate};
use std::f64;

/**
 * Basic arrow entity which is used for drawing the direction and power in the
 * shooting mode
 */
pub struct Arrow {
    pub position: Point2<f64>,
    pub direction: Vector2<f64>,
    pub length: f64,
    pub mode: ShootingMode,
    pub time_passed: f64,
}

/**
 * Mode to distinguish between the different staged of user interaction with the
 * white poolball. In rotate mode the used decides the direction whereas in
 * Power mode the user decides the poolball's initial speed
 */
#[derive(Clone, Debug, PartialEq)]
pub enum ShootingMode {
    Rotate,
    Power,
}

const ROTATION_SPEED: f64 = 1.0;
const MAX_LENGTH: f64 = 0.2;
const DEFAULT_LENGTH: f64 = 0.1;

impl Arrow {
    /**
     * Constructs a new Blackhole with the given properties
     */
    pub fn new(position: Point2<f64>) -> Arrow {
        Arrow {
            position: position,
            direction: Vector2::new(1.0, 1.0).normalize(),
            length: DEFAULT_LENGTH,
            mode: ShootingMode::Rotate,
            time_passed: 0.0,
        }
    }

    /**
     * Updates the arrow position, direction and length
     */
    pub fn update(&mut self, delta_time: f64) {
        match self.mode {
            // Rotates the arrow indicator
            ShootingMode::Rotate => {
                let angle = Vector1::new(ROTATION_SPEED) * delta_time;
                let rotation = Rotation2::new(angle);
                self.direction = rotation.rotate(&self.direction);

                // Reset time and length for new power round
                self.time_passed = 0.0;
                self.length = DEFAULT_LENGTH;
            }
            // Changes the size of the arrow indicator
            ShootingMode::Power => {
                self.time_passed += delta_time;
                let new_length = MAX_LENGTH * (self.time_passed).sin().abs();
                self.length = new_length;
            }
        }
    }

    /**
     * Renders itself using the given graphics
     */
    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics) {

        const MAGENTA: [f32; 4] = [1.0, 0.0, 1.0, 1.0];

        let line = line::Line::new(MAGENTA, 0.001);
        gl.draw(args.viewport(), |c, gl| {

            let trans = c.transform
                .scale(args.width as f64, args.height as f64)
                .trans(self.position.x, self.position.y);

            // Draw the line
            let pos_line =
                [0.0, 0.0, self.direction.x * self.length, self.direction.y * self.length];
            line.draw_arrow(pos_line, 0.1 * self.length, &c.draw_state, trans, gl);
        });
    }
}
