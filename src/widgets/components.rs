use bevy::prelude::*;

use super::TerminalWidget;

/// Component representing a terminal widget.
#[derive(Component)]
pub struct Widget {
    /// The widget instance itself, containing rendering and input logic
    pub widget: Box<dyn TerminalWidget + Send + Sync>,
    /// Depth to render widget at
    pub depth: u32,
    /// Whether this widget is currently enabled or should be hidden
    pub enabled: bool,
}
