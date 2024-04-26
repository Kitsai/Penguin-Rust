use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::EventPump;

use crate::arith::vec::*;

pub struct InputManager {
    _event_pump: EventPump,

    _mouse_pos: vec2::Vec2,
    _mouse_state: [bool; 6],
    _mouse_update: [usize; 6],

    _key_state: [bool; 416],
    _key_update: [usize; 416],

    _quit: bool,

    _update_count: usize,
}

impl InputManager {
    pub fn new(event_pump: EventPump) -> Self {

        Self {
            _event_pump: event_pump,

            _mouse_pos: vec2::Vec2::new(0.0, 0.0),
            _mouse_state: [false; 6],
            _mouse_update: [0; 6],

            _key_state: [false; 416],
            _key_update: [0; 416],

            _quit: false,

            _update_count: 0,
        }
    }

    pub fn update(&mut self) {
        self._update_count += 1;

        let ms = self._event_pump.mouse_state();
        self._mouse_pos = vec2::Vec2::new(ms.x() as f32, ms.y() as f32);

        for event in self._event_pump.poll_iter() {
            match event {
                Event::Quit {..} => self._quit = true,
                Event::MouseButtonDown { mouse_btn, ..} => {
                    self._mouse_state[mouse_btn as usize] = true;
                    self._mouse_update[mouse_btn as usize] = self._update_count;
                }

                Event::MouseButtonUp { mouse_btn, ..} => {
                    self._mouse_state[mouse_btn as usize] = false;
                    self._mouse_update[mouse_btn as usize] = self._update_count;
                }

                Event::KeyDown { keycode, repeat, .. } => {
                    if !repeat {
                        let mut id = match keycode {Some(k) => k as usize, None => 0};

                        if id >= 0x40000000 {
                            id -= 0x3FFFFF81;
                        }

                        self._key_state[id] = true;
                        self._key_update[id] = self._update_count;                
                    }
                } 

                Event::KeyUp { keycode, repeat, .. } => {
                    if !repeat {
                        let mut id = match keycode {Some(k) => k as usize, None => 0};

                        if id >= 0x40000000 {
                            id -= 0x3FFFFF81;
                        }

                        self._key_state[id] = false;
                        self._key_update[id] = self._update_count;                
                    }
                }   
                
                _ => {}
            }
        }
    }

    pub fn get_mouse_pos(&self) -> vec2::Vec2 {
        self._mouse_pos
    }

    pub fn quit(&self) -> bool {
        self._quit
    }
    

    pub fn mouse_press(&self, button: MouseButton) -> bool {
        self._mouse_state[button as usize] && self._mouse_update[button as usize] == self._update_count
    }

    pub fn mouse_release(&self, button: MouseButton) -> bool {
        !self._mouse_state[button as usize] && self._mouse_update[button as usize] == self._update_count
    }

    pub fn mouse_is_down(&self, button: MouseButton) -> bool {
        self._mouse_state[button as usize]
    }

    pub fn key_press(&self, key: Keycode) -> bool {
        let mut id = key as usize;

        if id >= 0x40000000 {
            id -= 0x3FFFFF81;
        }

        self._key_state[id] && self._key_update[id] == self._update_count
    }

    pub fn key_release(&self, key: Keycode) -> bool {
        let mut id = key as usize;

        if id >= 0x40000000 {
            id -= 0x3FFFFF81;
        }

        !self._key_state[id] && self._key_update[id] == self._update_count
    }

    pub fn key_is_down(&self, key: Keycode) -> bool {
        let mut id = key as usize;

        if id >= 0x40000000 {
            id -= 0x3FFFFF81;
        }

        self._key_state[id]
    }
}