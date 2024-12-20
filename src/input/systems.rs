use std::cmp::Ordering;

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use crossterm::event::{read, Event, KeyEvent, KeyEventKind, MediaKeyCode, ModifierKeyCode};
use smol_str::SmolStr;

use super::{
    components::DummyWindow,
    events::TerminalInputEvent,
    resources::EventQueue,
};

/// Initializes event queue and thread
pub fn setup_input(mut commands: Commands, event_queue: Res<EventQueue>) {
    commands.spawn(DummyWindow);
    let event_queue = event_queue.0.clone();
    std::thread::spawn(move || {
        loop {
            // `read()` blocks until an `Event` is available
            match read() {
                Ok(event) => {
                    event_queue.lock().unwrap().push(event);
                }
                Err(err) => {
                    panic!("Error reading input events: {:?}", err);
                }
            }
        }
    });
}

/// Reads events from queue and broadcasts corresponding `TerminalInputEvent`s
pub fn input_handling(
    event_queue: Res<EventQueue>,
    dummy_window_query: Query<Entity, With<DummyWindow>>,
    mut terminal_event_writer: EventWriter<TerminalInputEvent>,
    mut key_event_writer: EventWriter<KeyboardInput>,
) {
    let mut event_queue = event_queue.0.lock().unwrap();
    let mut key_events = Vec::<KeyEvent>::new();
    while let Some(event) = event_queue.pop() {
        if let Event::Key(event) = event {
            key_events.push(event);
        }
        terminal_event_writer.send(TerminalInputEvent(event));
    }

    key_events.sort_by(|&a, &b| a.kind.partial_cmp(&b.kind).unwrap_or(Ordering::Equal));
    let window = dummy_window_query.single();
    for event in key_events {
        if let Some(key_code) = crossterm_keycode_to_bevy_keycode(event.code) {
            if let Some(logical_key) = crossterm_keycode_to_bevy_key(event.code) {
                match event.kind {
                    KeyEventKind::Press => {
                        key_event_writer.send(KeyboardInput {
                            key_code,
                            logical_key,
                            state: ButtonState::Pressed,
                            window,
                            repeat: false,
                        });
                    }
                    KeyEventKind::Repeat => {
                        key_event_writer.send(KeyboardInput {
                            key_code,
                            logical_key,
                            state: ButtonState::Pressed,
                            window,
                            repeat: true,
                        });
                    }
                    KeyEventKind::Release => {
                        key_event_writer.send(KeyboardInput {
                            key_code,
                            logical_key,
                            state: ButtonState::Released,
                            window,
                            repeat: false,
                        });
                    }
                }
            }
        }
    }
}

fn crossterm_keycode_to_bevy_keycode(
    crossterm_keycode: crossterm::event::KeyCode,
) -> Option<bevy::input::keyboard::KeyCode> {
    use bevy::input::keyboard::KeyCode as BKey;
    use crossterm::event::KeyCode as CKey;
    match crossterm_keycode {
        CKey::Backspace => Some(BKey::Backspace),
        CKey::Enter => Some(BKey::Enter),
        CKey::Left => Some(BKey::ArrowLeft),
        CKey::Right => Some(BKey::ArrowRight),
        CKey::Up => Some(BKey::ArrowUp),
        CKey::Down => Some(BKey::ArrowDown),
        CKey::Home => Some(BKey::Home),
        CKey::End => Some(BKey::End),
        CKey::PageUp => Some(BKey::PageUp),
        CKey::PageDown => Some(BKey::PageDown),
        CKey::Tab | CKey::BackTab => Some(BKey::Tab),
        CKey::Delete => Some(BKey::Delete),
        CKey::Insert => Some(BKey::Insert),
        CKey::F(num) => match num {
            1 => Some(BKey::F1),
            2 => Some(BKey::F2),
            3 => Some(BKey::F3),
            4 => Some(BKey::F4),
            5 => Some(BKey::F5),
            6 => Some(BKey::F6),
            7 => Some(BKey::F7),
            8 => Some(BKey::F8),
            9 => Some(BKey::F9),
            10 => Some(BKey::F10),
            11 => Some(BKey::F11),
            12 => Some(BKey::F12),
            13 => Some(BKey::F13),
            14 => Some(BKey::F14),
            15 => Some(BKey::F15),
            16 => Some(BKey::F16),
            17 => Some(BKey::F17),
            18 => Some(BKey::F18),
            19 => Some(BKey::F19),
            20 => Some(BKey::F20),
            21 => Some(BKey::F21),
            22 => Some(BKey::F22),
            23 => Some(BKey::F23),
            24 => Some(BKey::F24),
            25 => Some(BKey::F25),
            26 => Some(BKey::F26),
            27 => Some(BKey::F27),
            28 => Some(BKey::F28),
            29 => Some(BKey::F29),
            30 => Some(BKey::F30),
            31 => Some(BKey::F31),
            32 => Some(BKey::F32),
            33 => Some(BKey::F33),
            34 => Some(BKey::F34),
            35 => Some(BKey::F35),
            _ => None,
        },
        CKey::Char(c) => match c {
            '1' | '!' => Some(BKey::Digit1),
            '2' | '@' => Some(BKey::Digit2),
            '3' | '#' => Some(BKey::Digit3),
            '4' | '$' => Some(BKey::Digit4),
            '5' | '%' => Some(BKey::Digit5),
            '6' | '^' => Some(BKey::Digit5),
            '7' | '&' => Some(BKey::Digit7),
            '8' | '*' => Some(BKey::Digit8),
            '9' | '(' => Some(BKey::Digit9),
            '0' | ')' => Some(BKey::Digit0),
            '-' | '_' => Some(BKey::Minus),
            '=' | '+' => Some(BKey::Equal),
            '`' | '~' => Some(BKey::Backquote),
            'q' | 'Q' => Some(BKey::KeyQ),
            'w' | 'W' => Some(BKey::KeyW),
            'e' | 'E' => Some(BKey::KeyE),
            'r' | 'R' => Some(BKey::KeyR),
            't' | 'T' => Some(BKey::KeyT),
            'y' | 'Y' => Some(BKey::KeyY),
            'u' | 'U' => Some(BKey::KeyU),
            'i' | 'I' => Some(BKey::KeyI),
            'o' | 'O' => Some(BKey::KeyO),
            'p' | 'P' => Some(BKey::KeyP),
            '[' | '{' => Some(BKey::BracketLeft),
            ']' | '}' => Some(BKey::BracketRight),
            'a' | 'A' => Some(BKey::KeyA),
            's' | 'S' => Some(BKey::KeyS),
            'd' | 'D' => Some(BKey::KeyD),
            'f' | 'F' => Some(BKey::KeyF),
            'g' | 'G' => Some(BKey::KeyG),
            'h' | 'H' => Some(BKey::KeyH),
            'j' | 'J' => Some(BKey::KeyJ),
            'k' | 'K' => Some(BKey::KeyK),
            'l' | 'L' => Some(BKey::KeyL),
            ';' | ':' => Some(BKey::Semicolon),
            '\'' | '"' => Some(BKey::Slash),
            'z' | 'Z' => Some(BKey::KeyZ),
            'x' | 'X' => Some(BKey::KeyX),
            'c' | 'C' => Some(BKey::KeyC),
            'v' | 'V' => Some(BKey::KeyV),
            'b' | 'B' => Some(BKey::KeyB),
            'n' | 'N' => Some(BKey::KeyN),
            'm' | 'M' => Some(BKey::KeyM),
            ',' | '<' => Some(BKey::Comma),
            '.' | '>' => Some(BKey::Period),
            '/' | '?' => Some(BKey::Slash),
            ' ' => Some(BKey::Space),
            _ => None,
        },
        CKey::Null => None,
        CKey::Esc => Some(BKey::Escape),
        CKey::CapsLock => Some(BKey::CapsLock),
        CKey::ScrollLock => Some(BKey::ScrollLock),
        CKey::NumLock => Some(BKey::NumLock),
        CKey::PrintScreen => Some(BKey::PrintScreen),
        CKey::Pause => Some(BKey::Pause),
        CKey::Menu => Some(BKey::ContextMenu),
        CKey::KeypadBegin => None,
        CKey::Media(media) => match media {
            MediaKeyCode::Play => Some(BKey::MediaPlayPause),
            MediaKeyCode::Pause => Some(BKey::Pause),
            MediaKeyCode::PlayPause => Some(BKey::MediaPlayPause),
            MediaKeyCode::Reverse => None,
            MediaKeyCode::Stop => Some(BKey::MediaStop),
            MediaKeyCode::FastForward => Some(BKey::MediaTrackNext),
            MediaKeyCode::Rewind => Some(BKey::MediaTrackPrevious),
            MediaKeyCode::TrackNext => Some(BKey::MediaTrackNext),
            MediaKeyCode::TrackPrevious => Some(BKey::MediaTrackPrevious),
            MediaKeyCode::Record => None,
            MediaKeyCode::LowerVolume => Some(BKey::AudioVolumeDown),
            MediaKeyCode::RaiseVolume => Some(BKey::AudioVolumeUp),
            MediaKeyCode::MuteVolume => Some(BKey::AudioVolumeMute),
        },
        CKey::Modifier(modifier) => match modifier {
            ModifierKeyCode::LeftShift => Some(BKey::ShiftLeft),
            ModifierKeyCode::LeftControl => Some(BKey::ControlLeft),
            ModifierKeyCode::LeftAlt => Some(BKey::AltLeft),
            ModifierKeyCode::LeftSuper => Some(BKey::SuperLeft),
            ModifierKeyCode::LeftHyper => Some(BKey::Hyper),
            ModifierKeyCode::LeftMeta => Some(BKey::Meta),
            ModifierKeyCode::RightShift => Some(BKey::ShiftRight),
            ModifierKeyCode::RightControl => Some(BKey::ControlRight),
            ModifierKeyCode::RightAlt => Some(BKey::AltLeft),
            ModifierKeyCode::RightSuper => Some(BKey::SuperRight),
            ModifierKeyCode::RightHyper => Some(BKey::Hyper),
            ModifierKeyCode::RightMeta => Some(BKey::Meta),
            ModifierKeyCode::IsoLevel3Shift => None,
            ModifierKeyCode::IsoLevel5Shift => None,
        },
    }
}

fn crossterm_keycode_to_bevy_key(
    crossterm_keycode: crossterm::event::KeyCode,
) -> Option<bevy::input::keyboard::Key> {
    use bevy::input::keyboard::Key as BKey;
    use crossterm::event::KeyCode as CKey;
    match crossterm_keycode {
        CKey::Backspace => Some(BKey::Backspace),
        CKey::Enter => Some(BKey::Enter),
        CKey::Left => Some(BKey::ArrowLeft),
        CKey::Right => Some(BKey::ArrowRight),
        CKey::Up => Some(BKey::ArrowUp),
        CKey::Down => Some(BKey::ArrowDown),
        CKey::Home => Some(BKey::Home),
        CKey::End => Some(BKey::End),
        CKey::PageUp => Some(BKey::PageUp),
        CKey::PageDown => Some(BKey::PageDown),
        CKey::Tab | CKey::BackTab => Some(BKey::Tab),
        CKey::Delete => Some(BKey::Delete),
        CKey::Insert => Some(BKey::Insert),
        CKey::F(num) => match num {
            1 => Some(BKey::F1),
            2 => Some(BKey::F2),
            3 => Some(BKey::F3),
            4 => Some(BKey::F4),
            5 => Some(BKey::F5),
            6 => Some(BKey::F6),
            7 => Some(BKey::F7),
            8 => Some(BKey::F8),
            9 => Some(BKey::F9),
            10 => Some(BKey::F10),
            11 => Some(BKey::F11),
            12 => Some(BKey::F12),
            13 => Some(BKey::F13),
            14 => Some(BKey::F14),
            15 => Some(BKey::F15),
            16 => Some(BKey::F16),
            17 => Some(BKey::F17),
            18 => Some(BKey::F18),
            19 => Some(BKey::F19),
            20 => Some(BKey::F20),
            21 => Some(BKey::F21),
            22 => Some(BKey::F22),
            23 => Some(BKey::F23),
            24 => Some(BKey::F24),
            25 => Some(BKey::F25),
            26 => Some(BKey::F26),
            27 => Some(BKey::F27),
            28 => Some(BKey::F28),
            29 => Some(BKey::F29),
            30 => Some(BKey::F30),
            31 => Some(BKey::F31),
            32 => Some(BKey::F32),
            33 => Some(BKey::F33),
            34 => Some(BKey::F34),
            35 => Some(BKey::F35),
            _ => None,
        },
        CKey::Char(c) => Some(BKey::Character(SmolStr::from(c.encode_utf8(&mut [0;4])))),
        CKey::Null => None,
        CKey::Esc => Some(BKey::Escape),
        CKey::CapsLock => Some(BKey::CapsLock),
        CKey::ScrollLock => Some(BKey::ScrollLock),
        CKey::NumLock => Some(BKey::NumLock),
        CKey::PrintScreen => Some(BKey::PrintScreen),
        CKey::Pause => Some(BKey::Pause),
        CKey::Menu => Some(BKey::ContextMenu),
        CKey::KeypadBegin => None,
        CKey::Media(media) => match media {
            MediaKeyCode::Play => Some(BKey::MediaPlayPause),
            MediaKeyCode::Pause => Some(BKey::Pause),
            MediaKeyCode::PlayPause => Some(BKey::MediaPlayPause),
            MediaKeyCode::Reverse => None,
            MediaKeyCode::Stop => Some(BKey::MediaStop),
            MediaKeyCode::FastForward => Some(BKey::MediaTrackNext),
            MediaKeyCode::Rewind => Some(BKey::MediaTrackPrevious),
            MediaKeyCode::TrackNext => Some(BKey::MediaTrackNext),
            MediaKeyCode::TrackPrevious => Some(BKey::MediaTrackPrevious),
            MediaKeyCode::Record => None,
            MediaKeyCode::LowerVolume => Some(BKey::AudioVolumeDown),
            MediaKeyCode::RaiseVolume => Some(BKey::AudioVolumeUp),
            MediaKeyCode::MuteVolume => Some(BKey::AudioVolumeMute),
        },
        CKey::Modifier(modifier) => match modifier {
            ModifierKeyCode::LeftShift => Some(BKey::Shift),
            ModifierKeyCode::LeftControl => Some(BKey::Control),
            ModifierKeyCode::LeftAlt => Some(BKey::Alt),
            ModifierKeyCode::LeftSuper => Some(BKey::Super),
            ModifierKeyCode::LeftHyper => Some(BKey::Hyper),
            ModifierKeyCode::LeftMeta => Some(BKey::Meta),
            ModifierKeyCode::RightShift => Some(BKey::Shift),
            ModifierKeyCode::RightControl => Some(BKey::Control),
            ModifierKeyCode::RightAlt => Some(BKey::Alt),
            ModifierKeyCode::RightSuper => Some(BKey::Super),
            ModifierKeyCode::RightHyper => Some(BKey::Hyper),
            ModifierKeyCode::RightMeta => Some(BKey::Meta),
            ModifierKeyCode::IsoLevel3Shift => Some(BKey::AltGraph),
            ModifierKeyCode::IsoLevel5Shift => None,
        },
    }
}
