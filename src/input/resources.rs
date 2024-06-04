use bevy::{prelude::*, utils::HashSet};
use crossterm::event::{Event, KeyCode};
use std::sync::{Arc, Mutex};

/// Resource containing currently pressed and released keys
#[derive(Resource, Default)]
pub struct TerminalInput {
    pressed_keys: HashSet<KeyCode>,
    released_keys: HashSet<KeyCode>,
}

impl TerminalInput {
    /// Gets whether the given key is pressed
    pub fn is_pressed(&self, code: KeyCode) -> bool {
        self.pressed_keys.contains(&code)
    }

    /// Gets whether the given key is released
    pub fn is_released(&self, code: KeyCode) -> bool {
        self.released_keys.contains(&code)
    }

    /// Sets given key to pressed
    pub(super) fn press(&mut self, code: KeyCode) {
        if !self.is_pressed(code) {
            self.pressed_keys.insert(code);
        }
    }

    /// Sets given key to released and removes pressed state
    pub(super) fn release(&mut self, code: KeyCode) {
        if self.is_pressed(code) {
            self.pressed_keys.remove(&code);
        }
        if !self.is_released(code) {
            self.released_keys.insert(code);
        }
    }

    /// Clears all released keys
    pub(super) fn clear_released(&mut self) {
        self.released_keys.clear();
    }
}

/// Event queue for crossterm input event thread
#[derive(Resource, Default)]
pub(crate) struct EventQueue(pub(super) Arc<Mutex<Vec<Event>>>);
