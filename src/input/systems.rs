use bevy::prelude::*;
use crossterm::event::{read, Event, KeyEventKind};

use super::{events::TerminalInputEvent, resources::{EventQueue, TerminalInput}};

/// Initializes event queue and thread
pub fn setup_input(event_queue: Res<EventQueue>) {
    let event_queue = event_queue.0.clone();
    std::thread::spawn(move || {
        loop {
            // `read()` blocks until an `Event` is available
            match read() {
                Ok(event) => {
                    event_queue.lock().unwrap().push(event);
                }
                Err(err) => {
                    panic!("Error reading input events: {:?}", err);
                }
            }
        }
    });
}

/// Reads events from queue and broadcasts corresponding `TerminalInputEvent`s
pub fn input_handling(
    event_queue: Res<EventQueue>,
    mut input: ResMut<TerminalInput>,
    mut event_writer: EventWriter<TerminalInputEvent>,
) {
    input.clear_released();
    let mut event_queue = event_queue.0.lock().unwrap();
    while let Some(event) = event_queue.pop() {
        if let Event::Key(event) = event {
            match event.kind {
                KeyEventKind::Press => {
                    input.press(event.code);
                }
                KeyEventKind::Release => {
                    input.release(event.code);
                }
                _ => (),
            }
        }
        event_writer.send(TerminalInputEvent(event));
    }
}
