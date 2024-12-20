use bevy::prelude::*;

use crate::input::events::TerminalInputEvent;

use super::components::Widget;

/// Invokes every enabled widget's `handle_events` methods for each incoming input event
pub fn widget_input_handling(
    mut widgets: Query<&mut Widget>,
    mut event_reader: EventReader<TerminalInputEvent>,
    mut commands: Commands,
) {
    for event in event_reader.read() {
        for mut widget in widgets.iter_mut().filter(|widget| widget.enabled) {
            widget.widget.handle_events(event, &mut commands);
        }
    }
}

pub fn update_widgets(mut widgets: Query<&mut Widget>, time: Res<Time>, mut commands: Commands) {
    for mut widget in widgets.iter_mut() {
        widget.widget.update(&time, &mut commands);
    }
}
