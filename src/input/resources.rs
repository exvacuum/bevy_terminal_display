use bevy::{prelude::*, utils::HashSet};
use crossterm::event::{Event, KeyCode};
use std::sync::{Arc, Mutex};

/// Resource containing currently pressed and released keys
#[derive(Resource, Default)]
pub struct TerminalInput {
    pressed_keys: HashSet<KeyCode>,
    just_pressed_keys: HashSet<KeyCode>,
    just_released_keys: HashSet<KeyCode>,
}

impl TerminalInput {
    /// Gets whether the given key is pressed
    pub fn is_pressed(&self, code: KeyCode) -> bool {
        self.pressed_keys.contains(&code)
    }

    /// Gets whether the given key was just pressed
    pub fn just_pressed(&self, code: KeyCode) -> bool {
        self.just_pressed_keys.contains(&code)
    }

    /// Gets whether the given key was just released
    pub fn just_released(&self, code: KeyCode) -> bool {
        self.just_released_keys.contains(&code)
    }

    /// Sets given key to pressed
    pub(super) fn press(&mut self, code: KeyCode) {
        if !self.pressed_keys.contains(&code) {
            self.pressed_keys.insert(code);
            self.just_pressed_keys.insert(code);
        }
    }

    /// Sets given key to released and removes pressed state
    pub(super) fn release(&mut self, code: KeyCode) {
        self.pressed_keys.remove(&code);
        self.just_released_keys.insert(code);
    }

    /// Clears all just released keys
    pub(super) fn clear_just_released(&mut self) {
        self.just_released_keys.clear();
    }

    /// Clears all just pressed keys
    pub(super) fn clear_just_pressed(&mut self) {
        self.just_pressed_keys.clear();
    }
}

/// Event queue for crossterm input event thread
#[derive(Resource, Default)]
pub(crate) struct EventQueue(pub(super) Arc<Mutex<Vec<Event>>>);
