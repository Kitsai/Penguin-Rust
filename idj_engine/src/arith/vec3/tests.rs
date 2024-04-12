use super::*;

#[test]
fn test_creation() {
    let v = Vec3::new(1.0,2.0,3.0);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 3.0);
}

#[test]
fn test_magnitude() {
    let v = Vec3::new(3.0, 4.0, 5.0);
    assert_eq!(v.sqr_magnitude(), 50.0);
    assert_eq!(v.magnitude(), 50.0_f32.sqrt());
}

#[test]
fn test_normalized() {
    let v = Vec3::new(3.0, 4.0, 5.0);
    let n = v.normalized();
    assert_eq!(n.x, 3.0/50.0_f32.sqrt());
    assert_eq!(n.y, 4.0/50.0_f32.sqrt());
    assert_eq!(n.z, 5.0/50.0_f32.sqrt());
    assert_eq!(n.sqr_magnitude(), 1.0);
}

#[test]
fn test_abs() {
    let v = Vec3::new(-1.0, 2.0, -3.0);
    let a = v.abs();
    assert_eq!(a.x, 1.0);
    assert_eq!(a.y, 2.0);
    assert_eq!(a.z, 3.0);
}

#[test]
fn test_add() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    let v3 = v1 + v2;
    assert_eq!(v3.x, 5.0);
    assert_eq!(v3.y, 7.0);
    assert_eq!(v3.z, 9.0);
}

#[test]
fn test_sub() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    let v3 = v1 - v2;
    assert_eq!(v3.x, -3.0);
    assert_eq!(v3.y, -3.0);
    assert_eq!(v3.z, -3.0);
}

#[test]
fn test_dot() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = 2.0f32;
    assert_eq!(v1 * v2, Vec3::new(2.0, 4.0, 6.0));
}