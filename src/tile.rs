use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};

use crate::{TilemapTiles, chunk::TilemapChunkPosition};

#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Tile;

#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct TileTextureIndex(pub u32);

#[derive(Component, Clone, Copy, Debug, Default, Deref, DerefMut, Reflect, PartialEq, Eq, Hash)]
#[reflect(Component)]
pub struct TilePosition(pub IVec2);

impl TilePosition {
    pub fn chunk_position(&self, tile_size: u32) -> TilemapChunkPosition {
        TilemapChunkPosition(self.0 / (tile_size as i32))
    }
}

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
    tiles.set(tile_position, entity);

    world.commands().entity(entity).insert(Name::new(format!(
        "Tile {},{}",
        tile_position.x, tile_position.y
    )));
}

fn on_remove_tile_of(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    dbg!("on_remove_tile_of");

    let tilemap_entity = world.get::<TileOf>(entity).unwrap().0;
    let tile_position = *world.get::<TilePosition>(entity).unwrap();

    let mut tiles = world.get_mut::<TilemapTiles>(tilemap_entity).unwrap();

    tiles.remove(tile_position);
}
