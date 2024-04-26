mod component;

use crate::arith::rect::Rect;
use component::Component;

pub struct GameObject {
    _components: Vec<Box<dyn Component>>,
    _is_dead: bool,
    _started: bool,
    pub hitbox: Rect,
    pub angle: f64,
}

impl GameObject {
    pub fn new() -> Self {
        GameObject {
            _components: Vec::new(),
            _is_dead: false,
            _started: false,
            hitbox: Rect::new(0.0, 0.0, 0.0, 0.0, 0.0),
            angle: 0.0,
        }
    }

    
}