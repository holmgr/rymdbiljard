use piston::input::RenderArgs;
use opengl_graphics::GlGraphics;
use graphics::Transformed;
use graphics::circle_arc;
use graphics::ellipse;
use graphics::radians::Radians;
use na::Point2;
use poolball;
use std::f64;

/**
 * Blackhole contains information about a single blackhole in the game, and
 * methods for rendering and interacting with other entities
 */
pub struct Blackhole {
    pub position: Point2<f64>,
    pub mass: f64,
    pub radius: f64,
    pub reach: f64,
}

impl Blackhole {
    /**
     * Constructs a new Blackhole with the given properties
     */
    pub fn new(position: Point2<f64>, mass: f64, radius: f64, reach: f64) -> Blackhole {
        Blackhole {
            position: position,
            mass: mass,
            radius: radius,
            reach: reach,
        }
    }

    /**
     * Returns `true` if the given poolball is in contact with the blackhole
     */
    pub fn is_spagettified(&self, poolball: &poolball::Poolball) -> bool {
        let distance = ((self.position.x - poolball.position.x).powi(2) +
                        (self.position.y - poolball.position.y).powi(2))
            .sqrt();
        return distance < self.radius + poolball.radius;
    }

    /**
     * Renders itself using the given graphics
     */
    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics) {

        const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];

        // Piston rs: very strange, rewrite
        let arc = circle_arc::CircleArc::new(YELLOW, 0.001, 0.001, Radians::_360());
        gl.draw(args.viewport(), |c, gl| {

            let trans = c.transform
                .scale(args.width as f64, args.height as f64)
                .trans(self.position.x, self.position.y);

            // Draw the cue ball
            arc.draw(ellipse::circle(0.0, 0.0, self.reach),
                     &c.draw_state,
                     trans,
                     gl);
        });
    }
}

#[test]
fn test_is_spagettified() {
    let bh = Blackhole::new(Point2::new(0.0, 0.0), 1.0, 1.0, 3.0);
    let pb1 = poolball::Poolball::new(Point2::new(1.0, 0.0), poolball::BallType::Red);
    let pb2 = poolball::Poolball::new(Point2::new(-1.0, 0.0), poolball::BallType::Red);
    let pb3 = poolball::Poolball::new(Point2::new(0.0, 1.0), poolball::BallType::Red);
    let pb4 = poolball::Poolball::new(Point2::new(0.0, -1.0), poolball::BallType::Red);
    let pb5 = poolball::Poolball::new(Point2::new(2.1, 0.0), poolball::BallType::Red);
    let pb6 = poolball::Poolball::new(Point2::new(0.0, 2.1), poolball::BallType::Red);
    assert!(bh.is_spagettified(&pb1));
    assert!(bh.is_spagettified(&pb2));
    assert!(bh.is_spagettified(&pb3));
    assert!(bh.is_spagettified(&pb4));
    assert!(!bh.is_spagettified(&pb5));
    assert!(!bh.is_spagettified(&pb6));
}
