use std::{
    io::{stdout, Write},
    usize,
};

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureFormat},
};
use crossterm::{
    cursor::{self, MoveTo},
    event::{read, Event, KeyEventKind, KeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    terminal::enable_raw_mode,
    ExecutableCommand, QueueableCommand,
};
use grex_framebuffer_extract::{
    components::FramebufferExtractDestination, render_assets::FramebufferExtractSource,
};

use crate::{
    events::TerminalInputEvent,
    resources::{EventQueue, TerminalInput},
};

const BRAILLE_CODE_MIN: u16 = 0x2800;
const BRAILLE_CODE_MAX: u16 = 0x28FF;
const BRAILLE_DOT_BIT_POSITIONS: [u8; 8] = [0, 1, 2, 6, 3, 4, 5, 7];

pub fn setup(event_queue: Res<EventQueue>) {
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

    let mut stdout = stdout();
    enable_raw_mode().expect("Failed to put terminal into raw mode");
    let _ = stdout.execute(PushKeyboardEnhancementFlags(
        KeyboardEnhancementFlags::REPORT_EVENT_TYPES,
    ));
    let _ = stdout.execute(cursor::Hide);
}

pub fn print_to_terminal(image_exports: Query<&FramebufferExtractDestination>) {
    for image_export in image_exports.iter() {
        let mut image = image_export
            .0
            .lock()
            .expect("Failed to get lock on output texture");
        //TODO: Find a better way of preventing first frame
        if image.size() == UVec2::ONE {
            continue;
        }
        if image.texture_descriptor.format != TextureFormat::R8Unorm {
            warn_once!("Extracted framebuffer texture is not R8Unorm format. Will attempt conversion, but consider changing your render texture's format.");
            info_once!("{:?}", image);
            match image.convert(TextureFormat::R8Unorm) {
                Some(img) => *image = img,
                None => error_once!(
                    "Could not convert to R8Unorm texture format. Unexpected output may occur."
                ),
            };
        }

        let mut output_buffer = Vec::<char>::new();
        let width = image.width();
        let height = image.height();
        let data = &image.data;
        for character_y in (0..height).step_by(4) {
            for character_x in (0..width).step_by(2) {
                let mut mask: u8 = 0;
                for offset_x in 0..2 {
                    for offset_y in 0..4 {
                        let x = character_x + offset_x;
                        let y = character_y + offset_y;
                        if x < width && y < height && data[(y * width + x) as usize] == 0xFF {
                            mask |= 1
                                << (BRAILLE_DOT_BIT_POSITIONS[(offset_x * 4 + offset_y) as usize]);
                        }
                    }
                }
                output_buffer.push(braille_char(mask));
            }
        }

        let string = output_buffer.into_iter().collect::<String>();
        let mut stdout = stdout();
        stdout.queue(MoveTo(0, 0)).unwrap();
        stdout.write_all(string.as_bytes()).unwrap();
        stdout.flush().unwrap();
    }
}

fn braille_char(mask: u8) -> char {
    match char::from_u32((BRAILLE_CODE_MIN + mask as u16) as u32) {
        Some(character) => {
            if character as u16 > BRAILLE_CODE_MAX {
                panic!("Number too big!")
            }
            character
        }
        None => panic!("Error converting character!"),
    }
}

pub fn input_handling(
    event_queue: Res<EventQueue>,
    mut input: ResMut<TerminalInput>,
    mut event_writer: EventWriter<TerminalInputEvent>,
) {
    let mut event_queue = event_queue.0.lock().unwrap();
    while let Some(event) = event_queue.pop() {
        if let Event::Key(event) = event {
            match event.kind {
                KeyEventKind::Press => {
                    input.press(event.code);
                }
                KeyEventKind::Release => {
                    input.release(event.code);
                }
                _ => (),
            }
        }
        event_writer.send(TerminalInputEvent(event));
    }
}

pub fn resize_handling(
    mut images: ResMut<Assets<Image>>,
    mut sources: ResMut<Assets<FramebufferExtractSource>>,
    mut event_reader: EventReader<TerminalInputEvent>,
) {
    for event in event_reader.read() {
        if let Event::Resize(w, h) = event.0 {
            for source in sources.iter_mut() {
                let image = images.get_mut(&source.1 .0).unwrap();
                image.resize(Extent3d {
                    width: w as u32 * 2,
                    height: h as u32 * 4,
                    depth_or_array_layers: 1,
                });
            }
        }
    }
}
