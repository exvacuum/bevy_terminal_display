use bevy::prelude::*;
use crossterm::event::Event;

/// An event triggered when a crossterm input event is received
#[derive(Event)]
pub struct TerminalInputEvent(pub Event);
