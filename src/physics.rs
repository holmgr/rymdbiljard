use std::f32;

//Calculates the gravity force from an object with given mass at given distance
fn gravity_acc(mass: f32, distance: f32) -> f32 {
	//let gravity_constant = 6.673 * (10.0_f32).powf(-11.0);
	//We use gravity_constant = 1 for now, as the actual constant would require very high mass
	let gravity_constant = 1.0;
	(gravity_constant * mass)/distance.powf(2.0)
}



//Basic tests for gravity_acc
#[test]
fn test_gravity_acc() {
  let acceleration = gravity_acc(1.0,1.0);
  assert_eq!(acceleration, 1.0);
  let acceleration = gravity_acc(1.0, 2.0);
  assert_eq!(acceleration, 0.25);
  let acceleration = gravity_acc(2.5, 2.0);
  assert_eq!(acceleration, 0.625);
}
