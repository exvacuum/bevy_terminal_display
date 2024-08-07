use bevy::{prelude::*, render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}};
use bevy_dither_post_process::components::DitherPostProcessSettings;
use bevy_headless_render::{components::{HeadlessRenderBundle, HeadlessRenderDestination}, render_assets::HeadlessRenderSource};

/// Marker component for terminal display
#[derive(Component)]
pub struct TerminalDisplay;

/// Bundle for terminal display, contains a handle to an image to be used as a render target to
/// render to the terminal
#[derive(Bundle)]
pub struct TerminalDisplayBundle {
    _terminal_display: TerminalDisplay,
    _headless_render_bundle: HeadlessRenderBundle,
    _dither_post_process_settings: DitherPostProcessSettings,
    image_handle: Handle<Image>,
}

impl TerminalDisplayBundle {
    /// Create a new terminal display with the given dither level. A higher level exponentially
    /// increases the size of the bayer matrix used in the ordered dithering calculations. If in
    /// doubt, 3 is a good starting value to test with.
    pub fn new(dither_level: u32, asset_server: &AssetServer) -> Self {
        let terminal_size = crossterm::terminal::size().unwrap();
        let size = Extent3d {
            width: (terminal_size.0 as u32) * 2,
            height: (terminal_size.1 as u32) * 4,
            depth_or_array_layers: 1,
        };

        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                dimension: TextureDimension::D2,
                format: TextureFormat::R8Unorm,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_SRC
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
            ..default()
        };

        image.resize(size);
        let image_handle = asset_server.add(image);

        let framebuffer_extract_source =
            asset_server.add(HeadlessRenderSource(image_handle.clone()));

        Self {
            _terminal_display: TerminalDisplay,
            _headless_render_bundle: HeadlessRenderBundle {
                source: framebuffer_extract_source,
                dest: HeadlessRenderDestination::default(),
            },
            image_handle,
            _dither_post_process_settings: DitherPostProcessSettings::new(
                dither_level,
                asset_server,
            ),
        }
    }

    /// Retrieves the handle to this display's target image. Anything written here will be
    /// displayed.
    pub fn image_handle(&self) -> Handle<Image> {
        self.image_handle.clone()
    }
}
