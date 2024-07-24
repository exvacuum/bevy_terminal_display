use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureFormat},
};
use bevy_headless_render::{components::HeadlessRenderDestination, render_assets::HeadlessRenderSource};
use crossterm::event::Event;
use ratatui::{
    style::Stylize,
    widgets::{Paragraph, Wrap},
};

use crate::{input::events::TerminalInputEvent, widgets::components::Widget};

use super::resources::Terminal;

const BRAILLE_CODE_MIN: u16 = 0x2800;
const BRAILLE_CODE_MAX: u16 = 0x28FF;

/// 0 3
/// 1 4
/// 2 5
/// 6 7
const BRAILLE_DOT_BIT_POSITIONS: [u8; 8] = [0, 1, 2, 6, 3, 4, 5, 7];

/// Prints out the contents of a render image to the terminal as braille characters
pub fn print_to_terminal(
    mut terminal: ResMut<Terminal>,
    image_exports: Query<&HeadlessRenderDestination>,
    mut widgets: Query<&mut Widget>,
) {
    for image_export in image_exports.iter() {
        let mut image = image_export
            .0
            .lock()
            .expect("Failed to get lock on output texture");
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
        terminal
            .0
            .draw(|frame| {
                frame.render_widget(
                    Paragraph::new(string)
                        .white()
                        .bold()
                        .wrap(Wrap { trim: true }),
                    frame.size(),
                );

                let mut active_widgets = widgets
                    .iter_mut()
                    .filter(|widget| widget.enabled)
                    .collect::<Vec<_>>();
                active_widgets.sort_by(|a, b| a.depth.cmp(&b.depth));
                for mut widget in active_widgets {
                    widget.widget.render(frame, frame.size());
                }
            })
            .expect("Failed to draw terminal frame");
    }
}

/// Utility function to convert a u8 into the corresponding braille character
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

/// Watches for terminal resize events and resizes the render image accordingly
pub fn resize_handling(
    mut images: ResMut<Assets<Image>>,
    mut sources: ResMut<Assets<HeadlessRenderSource>>,
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
