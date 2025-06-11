use std::convert::Infallible;

use bevy::{
    asset::{
        AssetLoader, LoadContext, RenderAssetUsages,
        io::{Reader, Writer},
        saver::{AssetSaver, SavedAsset},
        transformer::{AssetTransformer, TransformedAsset},
    },
    image::TextureFormatPixelInfo,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension},
};
use glob::glob;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A component representing a tileset image containing all tile textures.
#[derive(Component, Clone, Debug, Default)]
pub struct Tileset {
    pub image: Handle<Image>,
    pub tile_size: UVec2,
}

#[derive(Serialize, Deserialize, Debug)]
struct TilesetDefinition {
    tiles: TilesDefinition,
}

#[derive(Serialize, Deserialize, Debug)]
enum TilesDefinition {
    Glob(String),
    Paths(Vec<String>),
    Atlas { image: String, tile_size: UVec2 },
}

/// A loader for tileset images.
#[derive(Default)]
pub struct TilesetLoader;

/// Errors that can occur when loading a tileset image.
#[derive(Debug, Error)]
pub enum TilesetLoaderError {
    /// An error occurred while reading the file.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// An error occurred while parsing the RON file.
    #[error(transparent)]
    RonSpannedError(#[from] ron::error::SpannedError),
    /// An error occurred while loading the image.
    #[error(transparent)]
    LoadDirectError(#[from] bevy::asset::LoadDirectError),
}

impl AssetLoader for TilesetLoader {
    type Asset = Image;
    type Settings = ();
    type Error = TilesetLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let definition: TilesetDefinition = ron::de::from_bytes(&bytes)?;

        let texture_image = match definition.tiles {
            TilesDefinition::Glob(glob_string) => {
                // TODO(cp): Don't hardcode assets path -- figure out how to get it from the AssetPlugin
                let mut images = Vec::new();
                for path in
                    glob(&format!("./assets/{}", glob_string)).expect("Failed to read glob pattern")
                {
                    let Ok(path) = path else {
                        continue;
                    };
                    let path = path.strip_prefix("assets/").unwrap();

                    let image = load_context
                        .loader()
                        .immediate()
                        .load::<Image>(path)
                        .await?
                        .take();

                    images.push(image);
                }
                convert_images_to_array(images)
            }
            TilesDefinition::Paths(paths) => {
                let mut images = Vec::new();
                for path in paths {
                    let image = load_context
                        .loader()
                        .immediate()
                        .load::<Image>(path)
                        .await?
                        .take();
                    images.push(image);
                }
                convert_images_to_array(images)
            }
            TilesDefinition::Atlas { image, tile_size } => {
                let image = load_context
                    .loader()
                    .immediate()
                    .load::<Image>(image)
                    .await?
                    .take();

                convert_atlas_to_array(&image, tile_size)
            }
        };

        Ok(texture_image)
    }

    fn extensions(&self) -> &[&str] {
        &[".tileset.ron"]
    }
}

/// A transformer for tileset images.
#[derive(Default)]
pub struct TilesetTransformer;

impl AssetTransformer for TilesetTransformer {
    type AssetInput = Image;
    type AssetOutput = Image;
    type Settings = ();
    type Error = Infallible;

    async fn transform<'a>(
        &'a self,
        asset: TransformedAsset<Self::AssetInput>,
        _: &'a Self::Settings,
    ) -> Result<TransformedAsset<Self::AssetOutput>, Self::Error> {
        Ok(asset)
    }
}

/// A saver for tileset images.
#[derive(Default)]
pub struct TilesetSaver;

impl AssetSaver for TilesetSaver {
    type Asset = Image;
    type Settings = ();
    type OutputLoader = TilesetLoader;
    type Error = std::io::Error;

    async fn save(
        &self,
        _writer: &mut Writer,
        _asset: SavedAsset<'_, Self::Asset>,
        _settings: &Self::Settings,
    ) -> Result<(), Self::Error> {
        // writer.write_all(asset.as_bytes()).await?;
        Ok(())
    }
}

fn convert_images_to_array(images: Vec<Image>) -> Image {
    let mut array_data = Vec::new();

    let num_layers = images.len();
    assert!(
        num_layers > 0,
        "Must provide at least one image for a tileset"
    );

    let tile_size = images[0].size();
    let format = images[0].texture_descriptor.format;

    for image in images {
        array_data.extend_from_slice(&image.data.as_ref().unwrap());
    }

    Image::new(
        Extent3d {
            width: tile_size.x,
            height: tile_size.y,
            depth_or_array_layers: num_layers as u32,
        },
        TextureDimension::D2,
        array_data,
        format,
        RenderAssetUsages::default(),
    )
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
