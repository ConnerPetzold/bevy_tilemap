use bevy::{
    platform_support::collections::{HashMap, hash_map::Entry},
    prelude::*,
};

use crate::{
    TileOf, TilePosition,
    chunk::{TilemapChunkOf, TilemapChunkPosition},
};

const DEFAULT_TILE_SIZE: u32 = 16;

#[derive(Component, Clone, Debug, Reflect)]
#[require(Transform, Visibility, TilemapTiles, TilemapChunks, Name = "Tilemap")]
#[reflect(Component, Default)]
pub struct Tilemap {
    pub tile_size: u32,
}

impl Default for Tilemap {
    fn default() -> Self {
        Self {
            tile_size: DEFAULT_TILE_SIZE,
        }
    }
}

#[derive(Component, Deref, Default, Reflect)]
#[relationship_target(relationship = TileOf)]
#[reflect(Component, Default)]
pub struct TilemapTiles {
    #[deref]
    #[relationship]
    entities: Vec<Entity>,

    lookup: HashMap<TilePosition, Entity>,
}

impl TilemapTiles {
    pub fn set(&mut self, tile_position: TilePosition, entity: Entity) {
        self.lookup.insert(tile_position, entity);
    }

    pub fn get(&self, tile_position: TilePosition) -> Option<Entity> {
        self.lookup.get(&tile_position).copied()
    }

    pub fn remove(&mut self, tile_position: TilePosition) {
        self.lookup.remove(&tile_position);
    }
}

#[derive(Component, Deref, Default, Reflect)]
#[relationship_target(relationship = TilemapChunkOf)]
#[reflect(Component, Default)]
pub struct TilemapChunks {
    #[deref]
    #[relationship]
    entities: Vec<Entity>,

    lookup: HashMap<TilemapChunkPosition, Entity>,
}

impl TilemapChunks {
    pub fn set(&mut self, tilemap_chunk_position: TilemapChunkPosition, entity: Entity) {
        self.lookup.insert(tilemap_chunk_position, entity);
    }

    pub fn get(&self, tilemap_chunk_position: TilemapChunkPosition) -> Option<Entity> {
        self.lookup.get(&tilemap_chunk_position).copied()
    }

    pub fn remove(&mut self, tilemap_chunk_position: TilemapChunkPosition) {
        self.lookup.remove(&tilemap_chunk_position);
    }

    pub fn entry(
        &mut self,
        tilemap_chunk_position: TilemapChunkPosition,
    ) -> Entry<'_, TilemapChunkPosition, Entity> {
        self.lookup.entry(tilemap_chunk_position)
    }
}

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub enum TilemapTexture {
    Atlas(Handle<Image>),
}

impl Default for TilemapTexture {
    fn default() -> Self {
        Self::Atlas(default())
    }
}
