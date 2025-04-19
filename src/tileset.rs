use bevy::{
    asset::{AssetLoader, LoadContext, RenderAssetUsages, io::Reader},
    image::{ImageLoader, TextureFormatPixelInfo},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A tileset image is an image that contains a grid of tiles.
#[derive(Asset, TypePath, Debug, Deref, DerefMut)]
pub struct TilesetImage(pub Image);

/// A loader for tileset images.
#[derive(Default)]
pub struct TilesetImageLoader;

/// Settings for a tileset image.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TilesetImageSettings {
    /// The size of each tile in the tileset.
    pub tile_size: UVec2,
}

/// Errors that can occur when loading a tileset image.
#[derive(Debug, Error)]
pub enum TilesetLoaderError {
    /// An error occurred while reading the file.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// An error occurred while loading the image.
    #[error(transparent)]
    LoadDirectError(#[from] bevy::asset::LoadDirectError),
}

impl AssetLoader for TilesetImageLoader {
    type Asset = TilesetImage;
    type Settings = TilesetImageSettings;
    type Error = TilesetLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let path = load_context.asset_path().clone_owned();
        let image = load_context
            .loader()
            .immediate()
            .with_reader(reader)
            .load::<Image>(path)
            .await?
            .take();

        Ok(TilesetImage(convert_atlas_to_array(
            &image,
            settings.tile_size,
        )))
    }

    fn extensions(&self) -> &[&str] {
        ImageLoader::SUPPORTED_FILE_EXTENSIONS
    }
}

fn convert_atlas_to_array(atlas: &Image, tile_size: UVec2) -> Image {
    let atlas_data = atlas.data.as_ref().unwrap();
    let atlas_size = atlas.size();
    let num_rows = atlas_size.y / tile_size.y;
    let num_cols = atlas_size.x / tile_size.x;
    let num_layers = num_rows * num_cols;

    let pixel_size_bytes = atlas.texture_descriptor.format.pixel_size();
    let src_row_pitch = atlas_size.x as usize * pixel_size_bytes;
    let dst_row_pitch = tile_size.x as usize * pixel_size_bytes;
    let tile_size_bytes = (tile_size.x * tile_size.y) as usize * pixel_size_bytes;

    let mut array_data = vec![0u8; tile_size_bytes * num_layers as usize];

    for layer in 0..num_layers {
        let tile_row = (layer / num_cols) as usize;
        let tile_col = (layer % num_cols) as usize;

        // Calculate base offset for this tile in the source atlas
        let tile_x_offset = tile_col * tile_size.x as usize * pixel_size_bytes;
        let tile_y_offset = tile_row * tile_size.y as usize * src_row_pitch;
        let tile_base_offset = tile_y_offset + tile_x_offset;

        // Copy each row of the tile
        for y in 0..tile_size.y as usize {
            let src_offset = tile_base_offset + (y * src_row_pitch);
            let dst_offset = (layer as usize * tile_size_bytes) + (y * dst_row_pitch);

            // Verify bounds before copying
            if src_offset + dst_row_pitch > atlas_data.len() {
                panic!(
                    "Source offset {} + row pitch {} exceeds atlas data length {} at layer {}, y {}",
                    src_offset,
                    dst_row_pitch,
                    atlas_data.len(),
                    layer,
                    y
                );
            }

            array_data[dst_offset..dst_offset + dst_row_pitch]
                .copy_from_slice(&atlas_data[src_offset..src_offset + dst_row_pitch]);
        }
    }

    Image::new(
        Extent3d {
            width: tile_size.x,
            height: tile_size.y,
            depth_or_array_layers: num_layers,
        },
        TextureDimension::D2,
        array_data,
        atlas.texture_descriptor.format,
        RenderAssetUsages::default(),
    )
}
