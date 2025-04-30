use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};

use crate::{TilemapTiles, chunk::TilemapChunkPosition};

/// Marker component for tiles.
#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Tile;

/// Texture index for a tile.
/// The index corresponds to the position in the tilemap's texture atlas.
#[derive(Component, Clone, Debug, Default, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct TileTextureIndex(pub u16);

/// Trait for types that can be used as a tile position.
pub trait TilePositioning: std::ops::Deref<Target = IVec2> {
    /// Calculates the chunk position that contains this tile position.
    fn chunk_position(&self, chunk_size: u32) -> TilemapChunkPosition {
        TilemapChunkPosition(self.div_euclid(IVec2::splat(chunk_size as i32)))
    }

    /// Calculates the position of the tile in the chunk, in pixel coordinates.
    fn position_in_chunk(&self, chunk_size: u32) -> UVec2 {
        self.rem_euclid(IVec2::splat(chunk_size as i32)).as_uvec2()
    }

    /// Calculates the index of the tile in the chunk.
    fn index_in_chunk(&self, chunk_size: u32) -> usize {
        let UVec2 { x, y } = self.position_in_chunk(chunk_size);
        (x + (chunk_size - 1 - y) * chunk_size) as usize
    }
}

/// Position of a tile in the tilemap, in tile coordinates.
#[derive(Component, Clone, Copy, Debug, Default, Deref, DerefMut, Reflect, PartialEq, Eq, Hash)]
#[reflect(Component)]
#[require(OldTilePosition)]
pub struct TilePosition(pub IVec2);

impl TilePositioning for TilePosition {}

/// When a tile is moved, we need to keep track of its old position so we can
/// determine if it has moved to a new chunk.
#[derive(Component, Clone, Copy, Debug, Default, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct OldTilePosition(pub IVec2);

impl TilePositioning for OldTilePosition {}

/// Marker component for tiles that need to be re-rendered.
#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct TileDirty;

/// Stores the tilemap entity that this tile belongs to.
#[derive(Component, Clone, Debug, Deref, DerefMut, Reflect)]
#[require(
    Tile,
    Name = "Tile",
    TilePosition,
    TileTextureIndex,
    Transform,
    Visibility
)]
#[relationship(relationship_target = TilemapTiles)]
#[reflect(Component)]
#[component(on_add = on_add_tile_of, on_remove = on_remove_tile_of)]
pub struct TileOf(pub Entity);

fn on_add_tile_of(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let tilemap_entity = world.get::<TileOf>(entity).unwrap().0;
    let tile_position = *world.get::<TilePosition>(entity).unwrap();

    let mut tiles = world.get_mut::<TilemapTiles>(tilemap_entity).unwrap();
    tiles.insert(tile_position, entity);

    world.commands().entity(entity).insert((
        Name::new(format!("Tile {},{}", tile_position.x, tile_position.y)),
        TileDirty,
    ));
}

fn on_remove_tile_of(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let tilemap_entity = world.get::<TileOf>(entity).unwrap().0;
    let tile_position = *world.get::<TilePosition>(entity).unwrap();

    let mut tiles = world.get_mut::<TilemapTiles>(tilemap_entity).unwrap();

    tiles.remove(&tile_position);
}
