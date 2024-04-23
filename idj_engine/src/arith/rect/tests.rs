use super::*;
use crate::arith::vec3::Vec3;

#[test]
fn test_creation() {
    let new = Rect::new(1.0,2.0,3.0,4.0);
    assert_eq!(new.x, 1.0);
    assert_eq!(new.y, 2.0);
    assert_eq!(new.w, 3.0);
    assert_eq!(new.h, 4.0);
}

#[test]
fn test_area() {
    let new = Rect::new(1.0,2.0,3.0,4.0);
    assert_eq!(new.area(),12.0);
}

#[test]
fn test_center() {
    let mut new = Rect::new(1.0,2.0,3.0,4.0);
    let c = new.center();
    assert_eq!(c , Vec3::new(2.5,4.0, 0.0));

    new.set_center(0,0);
    let c = new.center();
    assert_eq!(c, Vec3::new(1.5,2.0,0.0));
}

#[test]
fn test_get_pos() {
    let new = Rect::new(1.0,2.0,3.0,4.0);
    assert_eq!(new.get_pos(), Vec3::new(1.0,2.0,0.0))
}