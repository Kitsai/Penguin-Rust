use std::rc::{Weak, Rc};

use crate::game_object::GameObject;

pub struct State {
    _game_objects: Vec<Rc<GameObject>>,
    _pop_requested: bool,
    _quit_requested: bool,
    _started: bool,
}

impl State {
    pub fn update(&mut self) {
        // TODO
    }
    pub fn render(&self) {
        // TODO
    }
    pub fn start(&mut self) {
        // TODO
        self._started = true;
    }
    pub fn pause(&mut self) {
        // TODO
    }
    pub fn resume(&mut self) {
        // TODO
    }

    pub fn add_object(&mut self, object: Rc<GameObject>) -> Weak<GameObject> {
        let weak  = Rc::downgrade(&object);
        self._game_objects.push(object);
        weak
    }

    pub fn remove_object(&mut self, object: Weak<GameObject>) {
        if let Some(object) = object.upgrade() {
            let index = self._game_objects.iter().position(|x| Rc::ptr_eq(x, &object));
            if let Some(index) = index {
                self._game_objects.remove(index);
            }
        }
    }

    pub fn pop_requested(&self) -> bool {
        self._pop_requested
    }

    pub fn quit_requested(&self) -> bool {
        self._quit_requested
    }
}