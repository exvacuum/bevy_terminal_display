#![warn(missing_docs)]

//! Bevy plugin which allows a camera to render to a terminal window.

use std::{
    fs::OpenOptions,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use bevy::{
    log::{
        tracing_subscriber::{self, prelude::*, Registry},
        Level, LogPlugin,
    },
    prelude::*,
    utils::tracing::level_filters::LevelFilter,
};
use bevy_dither_post_process::DitherPostProcessPlugin;
use bevy_framebuffer_extract::FramebufferExtractPlugin;

pub use crossterm;
use once_cell::sync::Lazy;
pub use ratatui;

/// Functions and types related to capture and display of world to terminal
pub mod display;

/// Functions and types related to capturing and processing user keyboard input
pub mod input;

/// Functions and types related to constructing and rendering TUI widgets
pub mod widgets;

static LOG_PATH: Lazy<Arc<Mutex<PathBuf>>> = Lazy::new(|| Arc::new(Mutex::new(PathBuf::default())));

/// Plugin providing terminal display functionality
pub struct TerminalDisplayPlugin {
    /// Path to redirect tracing logs to. Defaults to "debug.log"
    pub log_path: PathBuf,
}

impl Default for TerminalDisplayPlugin {
    fn default() -> Self {
        Self {
            log_path: "debug.log".into(),
        }
    }
}

impl Plugin for TerminalDisplayPlugin {
    fn build(&self, app: &mut App) {
        *LOG_PATH
            .lock()
            .expect("Failed to get lock on log path mutex") = self.log_path.clone();
        app.add_plugins((
            DitherPostProcessPlugin,
            FramebufferExtractPlugin,
            LogPlugin {
                update_subscriber: Some(|_| {
                    let log_file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(
                            LOG_PATH
                                .lock()
                                .expect("Failed to get lock on log path mutex")
                                .clone(),
                        )
                        .unwrap();
                    let file_layer = tracing_subscriber::fmt::Layer::new()
                        .with_writer(log_file)
                        .with_filter(LevelFilter::from_level(Level::INFO));
                    Box::new(Registry::default().with(file_layer))
                }),
                ..Default::default()
            },
        ))
        .add_systems(Startup, input::systems::setup_input)
        .add_systems(
            Update,
            (
                input::systems::input_handling,
                display::systems::resize_handling,
                display::systems::print_to_terminal,
                widgets::systems::widget_input_handling,
            ),
        )
        .insert_resource(display::resources::Terminal::default())
        .insert_resource(input::resources::EventQueue::default())
        .insert_resource(input::resources::TerminalInput::default())
        .add_event::<input::events::TerminalInputEvent>();
    }
}
