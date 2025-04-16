#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

use bevy::{asset::load_internal_asset, prelude::*, sprite::Material2dPlugin};

mod chunk;
mod tile;
mod tilemap;

pub use chunk::*;
pub use tile::*;
pub use tilemap::*;

/// A Bevy plugin that provides tilemap functionality.
/// This plugin adds the necessary systems and resources for managing and rendering tilemaps.
#[derive(Default)]
pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            TILEMAP_CHUNK_MATERIAL_SHADER_HANDLE,
            "chunk.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(Material2dPlugin::<TilemapChunkMaterial>::default())
            .register_type::<TileOf>()
            .register_type::<Tilemap>()
            .register_type::<TilemapTiles>()
            .register_type::<TilemapTexture>()
            .add_systems(Update, update_chunks);
    }
}

/// A prelude module that re-exports all public items from the crate.
/// This is useful for importing all commonly used items at once.
pub mod prelude {
    pub use super::*;
}
