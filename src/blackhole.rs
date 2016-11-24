use na::Point2;
use poolball;
use std::f64;

pub struct Blackhole {
    pub position: Point2<f64>,
    pub mass: f64,
    pub radius: f64,
    pub reach: f64,
}

impl Blackhole {
    // Creates a new Blackhole with initial position (Point2 with x and y cocoridnate),
    // mass, radius (of the hole) and reach for the gravity
    pub fn new(position: Point2<f64>, mass: f64, radius: f64, reach: f64) -> Blackhole {
        Blackhole {
            position: position,
            mass: mass,
            radius: radius,
            reach: reach,
        }
    }

    pub fn is_spagettified(&self, poolball: &poolball::Poolball) -> bool {
        let distance = ((self.position.x - poolball.position.x).powi(2) +
                        (self.position.y - poolball.position.y).powi(2))
            .sqrt();
        return distance < self.radius + poolball.radius;
    }
}

#[test]
fn test_is_spagettified() {
    let bh = Blackhole::new(Point2::new(0.0, 0.0), 1.0, 1.0, 3.0);
    let pb1 = poolball::Poolball::new(Point2::new(1.0, 0.0));
    let pb2 = poolball::Poolball::new(Point2::new(-1.0, 0.0));
    let pb3 = poolball::Poolball::new(Point2::new(0.0, 1.0));
    let pb4 = poolball::Poolball::new(Point2::new(0.0, -1.0));
    let pb5 = poolball::Poolball::new(Point2::new(2.1, 0.0));
    let pb6 = poolball::Poolball::new(Point2::new(0.0, 2.1));
    assert!(bh.is_spagettified(&pb1));
    assert!(bh.is_spagettified(&pb2));
    assert!(bh.is_spagettified(&pb3));
    assert!(bh.is_spagettified(&pb4));
    assert!(!bh.is_spagettified(&pb5));
    assert!(!bh.is_spagettified(&pb6));
}
