use std::sync::{Arc, Mutex};

use bevy::{prelude::*, utils::HashSet};
use crossterm::event::{Event, KeyCode};

#[derive(Resource, Default)]
pub struct TerminalInput {
    pressed_keys: HashSet<KeyCode>,
    released_keys: HashSet<KeyCode>,
}

impl TerminalInput {
    pub fn is_pressed(&self, code: KeyCode) -> bool {
        self.pressed_keys.contains(&code)
    }

    pub fn is_released(&self, code: KeyCode) -> bool {
        self.released_keys.contains(&code)
    }

    pub(super) fn press(&mut self, code: KeyCode) {
        if !self.is_pressed(code) {
            self.pressed_keys.insert(code);
        }
    }

    pub(super) fn release(&mut self, code: KeyCode) {
        if self.is_pressed(code) {
            self.pressed_keys.remove(&code);
        }
        if !self.is_released(code) {
            self.released_keys.insert(code);
        }
    }
}

#[derive(Resource, Default)]
pub(super) struct EventQueue(pub(super) Arc<Mutex<Vec<Event>>>);
