use na::{Point2};
use poolball;
use std::f64;

pub struct Blackhole {
	pub Point2<f64> pos,
	pub f64 mass,
	pub f64 radius,
	pub f64 reach,
}

impl Blackhole {
	//Creates a new Blackhole with initial pos (Point2 with x and y cocoridnate), mass ,radious (of the hole) and reach for the gravity
	pub fn new(pos: Point2<f64>, mass: f64, radius: f64, reach: f64) {
		Blackhole {
			pos: pos,
			mass: mass,
			radius: radius,
			reach: reach,
		}
	}

	pub fn isSpagettified(poolball: poolball::Poolball) -> bool {
		let distance = ((self.pos.x - poolball.position.x).powi(2) +(self.pos.y - poolball.position.y).powi(2)).sqrt()
		return distance < self.radius+poolball.radius;
	}
}