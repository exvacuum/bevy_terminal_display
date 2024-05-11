use std::{
    io::{stdout, Stdout},
    sync::{Arc, Mutex},
};

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet, Uuid},
};
use crossterm::{
    event::{
        EnableMouseCapture, Event, KeyCode, KeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    terminal::{enable_raw_mode, EnterAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, layout::Rect, Frame};

use crate::events::TerminalInputEvent;

#[derive(Resource, Default)]
pub struct TerminalInput {
    pressed_keys: HashSet<KeyCode>,
    released_keys: HashSet<KeyCode>,
}

impl TerminalInput {
    pub fn is_pressed(&self, code: KeyCode) -> bool {
        self.pressed_keys.contains(&code)
    }

    pub fn is_released(&self, code: KeyCode) -> bool {
        self.released_keys.contains(&code)
    }

    pub(super) fn press(&mut self, code: KeyCode) {
        if !self.is_pressed(code) {
            self.pressed_keys.insert(code);
        }
    }

    pub(super) fn release(&mut self, code: KeyCode) {
        if self.is_pressed(code) {
            self.pressed_keys.remove(&code);
        }
        if !self.is_released(code) {
            self.released_keys.insert(code);
        }
    }
}

#[derive(Resource, Default)]
pub(super) struct EventQueue(pub(super) Arc<Mutex<Vec<Event>>>);

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

#[derive(Resource, Default)]
pub struct TerminalUI {
    widgets: HashMap<Uuid, Box<dyn TerminalWidget + Sync + Send>>,
}

impl TerminalUI {
    pub fn insert_widget(&mut self, widget: Box<dyn TerminalWidget + Sync + Send>) -> Uuid {
        let id = Uuid::new_v4();
        self.widgets.insert(id, widget);
        id
    }

    pub fn get_widget(&mut self, id: Uuid) -> Option<&mut Box<dyn TerminalWidget + Sync + Send>> {
        self.widgets.get_mut(&id)
    }

    pub fn destroy_widget(&mut self, id: Uuid) {
        self.widgets.remove(&id);
    }

    pub fn widgets(&mut self) -> Vec<&mut Box<dyn TerminalWidget + Sync + Send>> {
        let mut vec = self.widgets.values_mut().collect::<Vec<_>>();
        vec.sort_by(|a, b| a.depth().cmp(&b.depth()).reverse());
        vec
    }
}

pub trait TerminalWidget {
    fn init(&mut self) {}
    fn update(&mut self) {}
    fn render(&mut self, frame: &mut Frame, rect: Rect);
    fn handle_events(&mut self, _event: &TerminalInputEvent, _commands: &mut Commands) {}
    fn depth(&self) -> u32 {
        0
    }
}
