use std::{io::stdout, fs::OpenOptions};

use bevy::{
    log::{
        tracing_subscriber::{self, Registry, prelude::*},
        LogPlugin, Level,
    },
    prelude::*, utils::tracing::level_filters::LevelFilter,
};
use crossterm::{
    event::{DisableMouseCapture, PopKeyboardEnhancementFlags},
    terminal::{disable_raw_mode, LeaveAlternateScreen},
    ExecutableCommand,
};
use grex_dither_post_process::DitherPostProcessPlugin;
use grex_framebuffer_extract::FramebufferExtractPlugin;

pub use crossterm;
pub use ratatui;

pub mod components;
pub mod events;
pub mod resources;
mod systems;

pub struct TerminalDisplayPlugin;

impl Plugin for TerminalDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DitherPostProcessPlugin,
            FramebufferExtractPlugin,
            LogPlugin {
                update_subscriber: Some(|_| {
                    let log_file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open("debug.log")
                        .unwrap();
                    let file_layer = tracing_subscriber::fmt::Layer::new()
                        .with_writer(log_file)
                        .with_filter(LevelFilter::from_level(Level::INFO));
                    Box::new(Registry::default().with(file_layer))
                }),
                ..Default::default()
            },
        ))
        .add_systems(Startup, systems::setup)
        .add_systems(
            Update,
            (
                systems::input_handling,
                systems::resize_handling,
                systems::print_to_terminal,
                systems::widget_input_handling,
            ),
        )
        .insert_resource(resources::Terminal::default())
        .insert_resource(resources::EventQueue::default())
        .insert_resource(resources::TerminalInput::default())
        .insert_resource(resources::TerminalUI::default())
        .add_event::<events::TerminalInputEvent>();
    }
}

impl Drop for TerminalDisplayPlugin {
    fn drop(&mut self) {
        let mut stdout = stdout();
        let _ = stdout.execute(PopKeyboardEnhancementFlags);
        let _ = stdout.execute(DisableMouseCapture);
        let _ = stdout.execute(LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}
