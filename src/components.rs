
use bevy::{
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};
use grex_dither_post_process::components::DitherPostProcessSettings;
use grex_framebuffer_extract::{
    components::{ExtractFramebufferBundle, FramebufferExtractDestination},
    render_assets::FramebufferExtractSource,
};

use crate::resources::TerminalWidget;

#[derive(Component)]
pub struct TerminalDisplay;

#[derive(Bundle)]
pub struct TerminalDisplayBundle {
    _terminal_display: TerminalDisplay,
    extract_framebuffer_bundle: ExtractFramebufferBundle,
    dither_post_process_settings: DitherPostProcessSettings,
    image_handle: Handle<Image>,
}

impl TerminalDisplayBundle {
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
            asset_server.add(FramebufferExtractSource(image_handle.clone()));

        Self {
            _terminal_display: TerminalDisplay,
            extract_framebuffer_bundle: ExtractFramebufferBundle {
                source: framebuffer_extract_source,
                dest: FramebufferExtractDestination::default(),
            },
            image_handle,
            dither_post_process_settings: DitherPostProcessSettings::new(
                dither_level,
                asset_server,
            ),
        }
    }

    pub fn image_handle(&self) -> Handle<Image> {
        self.image_handle.clone()
    }
}

#[derive(Component)]
pub struct Widget {
    pub widget: Box<dyn TerminalWidget + Send + Sync>,
    pub depth: u32,
    pub enabled: bool,
}

#[derive(Component)]
pub struct Tooltip(pub String);
