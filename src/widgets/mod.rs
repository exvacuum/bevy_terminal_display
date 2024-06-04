use bevy::prelude::*;
use downcast_rs::{impl_downcast, DowncastSync};
use ratatui::{layout::Rect, Frame};

use crate::input::events::TerminalInputEvent;

/// Components for this module
pub mod components;

/// Systems for this module
pub(crate) mod systems;

/// Trait which defines an interface for terminal widgets
pub trait TerminalWidget: DowncastSync {
    /// Called every frame to render the widget
    fn render(&mut self, frame: &mut Frame, rect: Rect);

    /// Called when a terminal input event is invoked to update any state accordingly
    fn handle_events(&mut self, _event: &TerminalInputEvent, _commands: &mut Commands) {}
}
impl_downcast!(sync TerminalWidget);
