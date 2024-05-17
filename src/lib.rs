use std::{fs::OpenOptions, io::stdout, path::PathBuf, sync::{Arc, Mutex}};

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
use once_cell::sync::Lazy;
pub use ratatui;

pub mod components;
pub mod events;
pub mod resources;
mod systems;

static LOG_PATH: Lazy<Arc<Mutex<PathBuf>>> = Lazy::new(|| Arc::new(Mutex::new(PathBuf::default())));

pub struct TerminalDisplayPlugin {
    pub log_path: PathBuf,
}

impl Default for TerminalDisplayPlugin {
    fn default() -> Self {
        Self {
            log_path: "debug.log".into()
        }
    }
}

impl Plugin for TerminalDisplayPlugin {
    fn build(&self, app: &mut App) {
        *LOG_PATH.lock().expect("Failed to get lock on log path mutex") = self.log_path.clone();
        app.add_plugins((
            DitherPostProcessPlugin,
            FramebufferExtractPlugin,
            LogPlugin {
                update_subscriber: Some(|_| {
                    let log_file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(LOG_PATH.lock().expect("Failed to get lock on log path mutex").clone())
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
