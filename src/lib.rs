use bevy::{asset::load_internal_asset, prelude::*, sprite::Material2dPlugin};

mod chunk;
mod tile;
mod tilemap;

use chunk::{TILEMAP_CHUNK_MATERIAL_SHADER_HANDLE, TilemapChunkMaterial, update_chunks};
pub use tile::*;
pub use tilemap::*;

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

pub mod prelude {
    pub use super::*;
    // pub use super::{Tile, Tilemap};
}
