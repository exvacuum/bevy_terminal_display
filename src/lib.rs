use std::io::stdout;

use bevy::prelude::*;
use crossterm::{
    event::PopKeyboardEnhancementFlags, terminal::disable_raw_mode, ExecutableCommand,
};
use grex_dither_post_process::DitherPostProcessPlugin;
use grex_framebuffer_extract::FramebufferExtractPlugin;

pub use crossterm::event::KeyCode;

pub mod components;
pub mod events;
pub mod resources;
mod systems;

pub struct TerminalDisplayPlugin;

impl Plugin for TerminalDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DitherPostProcessPlugin, FramebufferExtractPlugin))
            .add_systems(Startup, systems::setup)
            .add_systems(
                Update,
                (
                    systems::input_handling,
                    systems::resize_handling,
                    systems::print_to_terminal,
                ),
            )
            .insert_resource(resources::EventQueue::default())
            .insert_resource(resources::TerminalInput::default())
            .add_event::<events::TerminalInputEvent>();
    }
}

impl Drop for TerminalDisplayPlugin {
    fn drop(&mut self) {
        let mut stdout = stdout();
        stdout.execute(PopKeyboardEnhancementFlags).unwrap();
        disable_raw_mode().unwrap();
    }
}
