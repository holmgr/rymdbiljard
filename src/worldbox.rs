use na::{Point2};
use std::f32;

pub struct WorldBox {
	pub ulcorner: Point2<f32>,
	pub lrconrner: Point2<f32>,
	pub center: Point2<f32>,
	pub newconrners: Vec<Point2<f32>>,
	pub rotationrad: f32,
}

impl WorldBox {
	pub fn new(upperleftcorner: Point2<f32>, lowerrightcorner: Point2<f32>, rotationrad: f32) -> WorldBox {
		let ll = Point2::new(upperleftcorner.x,lowerrightcorner.y);
		let ur = Point2::new(lowerrightcorner.x,upperleftcorner.y);
		let center = Point2::new(((lowerrightcorner.x-upperleftcorner.x)/2.0)+upperleftcorner.x,((lowerrightcorner.y-upperleftcorner.y)/2.0)+upperleftcorner.y);
		let mut newconrners: Vec<Point2<f32>> = vec![];
		newconrners.push(calculate_corner_rotation(center.x,center.y,upperleftcorner.x,upperleftcorner.y,rotationrad));
		newconrners.push(calculate_corner_rotation(center.x,center.y,ur.x,ur.y,rotationrad));
		newconrners.push(calculate_corner_rotation(center.x,center.y,ll.x,ll.y,rotationrad));
		newconrners.push(calculate_corner_rotation(center.x,center.y,lowerrightcorner.x,lowerrightcorner.y,rotationrad));
		WorldBox {
			ulcorner: upperleftcorner,
			lrconrner: lowerrightcorner,
			center: center,
			newconrners: newconrners,
			rotationrad: rotationrad
		}
	}
}

fn calculate_corner_rotation(xcenter: f32,ycenter: f32,x: f32,y: f32, rotationrad: f32) -> Point2<f32> {
	let p = Point2::new(xcenter+(x-xcenter)*rotationrad.cos()+(y-ycenter)*rotationrad.sin(), 
		ycenter+(x-xcenter)*rotationrad.sin()+(y-ycenter)*rotationrad.cos());
	print!("the roration {} cos: {} sin: {}",rotationrad, rotationrad.cos(), rotationrad.sin());
	return p;
}


#[test]
fn test_calculate_corner_rotation() {
	let tmp = calculate_corner_rotation(0.0,0.0,1.0,0.0,f32::consts::FRAC_PI_2);
	assert!((tmp.x).abs() < f32::EPSILON);
	assert!((1.0-tmp.y).abs() < f32::EPSILON);
}

