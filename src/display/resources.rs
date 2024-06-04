use std::io::{stdout, Stdout};

use bevy::prelude::*;
use crossterm::{
    event::{
        DisableMouseCapture, EnableMouseCapture, KeyboardEnhancementFlags,
        PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::backend::CrosstermBackend;

/// Ratatui terminal instance. Enters alternate screen when constructed, and exits once dropped.
#[derive(Resource)]
pub struct Terminal(pub ratatui::Terminal<CrosstermBackend<Stdout>>);

impl Default for Terminal {
    fn default() -> Self {
        stdout().execute(EnterAlternateScreen).unwrap();
        stdout().execute(EnableMouseCapture).unwrap();
        stdout()
            .execute(PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::REPORT_EVENT_TYPES,
            ))
            .unwrap();
        enable_raw_mode().unwrap();
        let mut terminal = ratatui::Terminal::new(CrosstermBackend::new(stdout()))
            .expect("Failed to create terminal");
        terminal.clear().expect("Failed to clear terminal");
        Self(terminal)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let mut stdout = stdout();
        let _ = stdout.execute(PopKeyboardEnhancementFlags);
        let _ = stdout.execute(DisableMouseCapture);
        let _ = stdout.execute(LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}
