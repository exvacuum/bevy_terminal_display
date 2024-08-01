#![warn(missing_docs)]

//! Bevy plugin which allows a camera to render to a terminal window.

use std::{
    fs::OpenOptions, io::stdout, path::PathBuf, sync::{Arc, Mutex}
};

use bevy::{
    log::{
        tracing_subscriber::{self, layer::SubscriberExt, EnvFilter, Layer, Registry},
        Level,
    }, prelude::*, utils::tracing::subscriber,
};
use bevy_dither_post_process::DitherPostProcessPlugin;

use bevy_headless_render::HeadlessRenderPlugin;
use color_eyre::config::HookBuilder;
pub use crossterm;
use crossterm::{event::{DisableMouseCapture, PopKeyboardEnhancementFlags}, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
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
            .with_filter(EnvFilter::builder().parse_lossy(format!("{},{}", Level::INFO, "wgpu=error,naga=warn")));
        let subscriber = Registry::default().with(file_layer);
        subscriber::set_global_default(subscriber).unwrap();

        let (panic, error) = HookBuilder::default().into_hooks();
        let panic = panic.into_panic_hook();
        let error = error.into_eyre_hook();

        color_eyre::eyre::set_hook(Box::new(move |e| {
            let _ = restore_terminal();
            error(e)
        })).unwrap();

        std::panic::set_hook(Box::new(move |info| {
            let _ = restore_terminal();
            panic(info);
        }));
        
        app.add_plugins((
            DitherPostProcessPlugin,
            HeadlessRenderPlugin,
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

fn restore_terminal() {
    let mut stdout = stdout();
    let _ = stdout.execute(PopKeyboardEnhancementFlags);
    let _ = stdout.execute(DisableMouseCapture);
    let _ = stdout.execute(LeaveAlternateScreen);
    let _ = disable_raw_mode();
}
