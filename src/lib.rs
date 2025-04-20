#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

use bevy::prelude::*;

mod chunk;
mod tile;
mod tilemap;
mod tileset;

pub use chunk::*;
pub use tile::*;
pub use tilemap::*;
pub use tileset::*;

/// A Bevy plugin that provides tilemap functionality.
/// This plugin adds the necessary systems and resources for managing and rendering tilemaps.
#[derive(Default)]
pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<TilesetLoader>()
            .register_type::<TileOf>()
            .register_type::<Tilemap>()
            .register_type::<TilemapTiles>()
            .add_systems(Update, update_chunks);
    }
}

/// A prelude module that re-exports all public items from the crate.
/// This is useful for importing all commonly used items at once.
pub mod prelude {
    pub use super::*;
}
