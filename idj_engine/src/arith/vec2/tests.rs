use super::Vec2;

#[test]
fn test_creation() {
    let v = Vec2::new(1.0, 2.0);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
}

#[test]
fn test_magnitude() {
    let v = Vec2::new(3.0, 4.0);
    assert_eq!(v.sqr_magnitude(), 25.0);
    assert_eq!(v.magnitude(), 5.0);
}

#[test]
fn test_normalized() {
    let v = Vec2::new(3.0, 4.0);
    let n = v.normalized();
    assert_eq!(n.x, 3.0/5.0);
    assert_eq!(n.y, 4.0/5.0);
    assert_eq!(n.sqr_magnitude(), 1.0);
}

#[test]
fn test_inclination() {
    let v = Vec2::new(1.0, 1.0);
    assert_eq!(v.inclination(), std::f32::consts::FRAC_PI_4);
}

#[test]
fn test_inclination_vec2() {
    let v1 = Vec2::new(1.0, 1.0);
    let v2 = Vec2::new(1.0, 0.0);
    assert_eq!(v1.inclination_vec2(&v2), std::f32::consts::FRAC_PI_2);
}

#[test]
fn test_rotated() {
    let v = Vec2::new(5.0, 0.0);
    let r = v.get_rotated(std::f32::consts::FRAC_PI_2);
    assert_eq!(r.y, 5.0);
}

#[test]
fn test_abs() {
    let v = Vec2::new(-1.0, 2.0);
    let a = v.abs();
    assert_eq!(a.x, 1.0);
    assert_eq!(a.y, 2.0);
}

#[test]
fn test_dot() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = 2.0f32;
    assert_eq!(v1 * v2, Vec2::new(2.0, 4.0));
}

#[test]
fn test_add() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(3.0, 4.0);
    let a = v1 + v2;
    assert_eq!(a.x, 4.0);
    assert_eq!(a.y, 6.0);
}

#[test]
fn test_sub() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(3.0, 4.0);
    let s = v1 - v2;
    assert_eq!(s.x, -2.0);
    assert_eq!(s.y, -2.0);
}

// Path: idj_engine/src/arith/vec3/tests.rs