use bevy::{platform_support::collections::HashMap, prelude::*};

use crate::{
    TileOf, TilePosition,
    chunk::{TilemapChunkOf, TilemapChunkPosition},
};

/// Default size of a chunk in tiles
const DEFAULT_CHUNK_SIZE: u32 = 64;
/// Default size of a tile in pixels
const DEFAULT_TILE_SIZE: u32 = 16;

/// Represents a tilemap in the game world.
/// The tilemap is divided into chunks for efficient rendering and management.
#[derive(Component, Clone, Debug, Reflect)]
#[require(Transform, Visibility, TilemapTiles, TilemapChunks, Name = "Tilemap")]
#[reflect(Component, Default)]
pub struct Tilemap {
    /// The size of each tile in pixels
    pub tile_size: u32,
    /// The size of each chunk in tiles
    pub chunk_size: u32,
}

impl Default for Tilemap {
    fn default() -> Self {
        Self {
            tile_size: DEFAULT_TILE_SIZE,
            chunk_size: DEFAULT_CHUNK_SIZE,
        }
    }
}

/// Stores all tiles in a tilemap.
/// Maintains a mapping between tile positions and their entities.
#[derive(Component, Deref, DerefMut, Default, Reflect)]
#[relationship_target(relationship = TileOf)]
#[reflect(Component, Default)]
pub struct TilemapTiles {
    #[relationship]
    entities: Vec<Entity>,

    #[deref]
    lookup: HashMap<TilePosition, Entity>,
}

/// Stores all chunks in a tilemap.
/// Maintains a mapping between chunk positions and their entities.
#[derive(Component, Deref, DerefMut, Default, Reflect)]
#[relationship_target(relationship = TilemapChunkOf)]
#[reflect(Component, Default)]
pub struct TilemapChunks {
    #[relationship]
    entities: Vec<Entity>,

    #[deref]
    lookup: HashMap<TilemapChunkPosition, Entity>,
}

/// Defines the texture used by a tilemap.
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub enum TilemapTexture {
    /// Texture atlas containing all tile textures
    Atlas(Handle<Image>),
}

impl Default for TilemapTexture {
    fn default() -> Self {
        Self::Atlas(default())
    }
}
