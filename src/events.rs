use bevy::prelude::*;
use crossterm::event::Event;

#[derive(Event)]
pub struct TerminalInputEvent(pub Event);
