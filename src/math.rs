use na::{Vector2, Point2, Norm, FloatPoint};

// Calculates the magnitude of the vector
pub fn calc_magnitude(vector: Vector2<f64>) -> f64 {
    return ((vector.x).powf(2.0) + (vector.y).powf(2.0)).sqrt();
}


#[test]
fn test_magnitude() {
    let vector = Vector2::new(1.0, 1.0);
    let magnitude = calc_magnitude(vector);
    assert_eq!(magnitude, (2.0_f64).sqrt());

    let vector2 = Vector2::new(1.0, 2.0);
    let magnitude2 = calc_magnitude(vector2);
    assert_eq!(magnitude2, (5.0_f64).sqrt());


    let vector3 = Vector2::new(2.0, 0.0);
    let magnitude3 = calc_magnitude(vector3);
    assert_eq!(magnitude3, 2.0);


}
