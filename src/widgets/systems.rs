use bevy::prelude::*;

use crate::{display::resources::Terminal, input::events::TerminalInputEvent};

use super::components::Widget;

/// Invokes every enabled widget's `render` method
pub fn draw_widgets(mut terminal: ResMut<Terminal>, mut widgets: Query<&mut Widget>) {
    terminal
        .0
        .draw(|frame| {
            let mut active_widgets = widgets
                .iter_mut()
                .filter(|widget| widget.enabled)
                .collect::<Vec<_>>();
            active_widgets.sort_by(|a, b| a.depth.cmp(&b.depth));
            for mut widget in active_widgets {
                widget.widget.render(frame, frame.size());
            }
        })
        .unwrap();
}

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
