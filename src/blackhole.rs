use na::{Point2};
use poolball;
use std::f64;

pub struct Blackhole {
	pub position: Point2<f64>,
	pub mass: f64,
	pub radius: f64,
	pub reach: f64,
}

impl Blackhole {
	//Creates a new Blackhole with initial position (Point2 with x and y cocoridnate), mass ,radious (of the hole) and reach for the gravity
	pub fn new(position: Point2<f64>, mass: f64, radius: f64, reach: f64) -> Blackhole{
		Blackhole {
			position: position,
			mass: mass,
			radius: radius,
			reach: reach,
		}
	}

	pub fn isSpagettified(&self, poolball: poolball::Poolball) -> bool {
		let distance = ((self.position.x - poolball.position.x).powi(2) +(self.position.y - poolball.position.y).powi(2)).sqrt();
		return distance < self.radius+poolball.radius;
	}
}