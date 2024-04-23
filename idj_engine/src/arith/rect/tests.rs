use super::*;
use crate::arith::vec3::Vec3;

#[test]
fn test_creation() {
    let new = Rect::new(1.0,2.0,3.0,4.0, 5.0);
    assert_eq!(new.x, 1.0);
    assert_eq!(new.y, 2.0);
    assert_eq!(new.z, 3.0);
    assert_eq!(new.w, 4.0);
    assert_eq!(new.h, 5.0);
}

#[test]
fn test_area() {
    let new = Rect::new(1.0,2.0,3.0,4.0, 5.0);
    assert_eq!(new.area(),20.0);
}

#[test]
fn test_center() {
    let mut new = Rect::new(1.0,2.0,3.0,4.0, 5.0);
    let c = new.center();
    assert_eq!(c , Vec3::new(3.0, 4.5, new.z));

    new.set_center(10.0,10.0);
    let c = new.center();
    assert_eq!(c, Vec3::new(10.0, 10.0, new.z));
}

#[test]
fn test_get_pos() {
    let new = Rect::new(1.0,2.0,3.0,4.0, 5.0);
    assert_eq!(new.get_pos(), Vec3::new(1.0,2.0,3.0))
}