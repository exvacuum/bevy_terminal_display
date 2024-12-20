use bevy::prelude::*;
use crossterm::event::Event;
use std::sync::{Arc, Mutex};

/// Event queue for crossterm input event thread
#[derive(Resource, Default)]
pub(crate) struct EventQueue(pub(super) Arc<Mutex<Vec<Event>>>);
