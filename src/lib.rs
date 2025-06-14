// #![deny(missing_docs)]
#![doc = include_str!("../README.md")]

use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    platform::collections::HashMap,
    prelude::*,
    sprite::AlphaMode2d,
};

mod storage;
mod tilemap_chunk;
mod tilemap_chunk_material;
mod tileset;

pub use storage::*;
pub use tilemap_chunk::*;
pub use tilemap_chunk_material::*;
pub use tileset::*;

/// A Bevy plugin that provides tilemap functionality.
/// This plugin adds the necessary systems and resources for managing and rendering tilemaps.
#[derive(Default)]
pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapChunkPlugin)
            .add_plugins(TilemapChunkMaterialPlugin)
            .init_asset_loader::<TilesetLoader>()
            .register_type::<TileOf>()
            .register_type::<TilemapTiles>()
            .add_systems(PreUpdate, sync_tiles);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum TilemapRenderMode {
    #[default]
    Orthogonal,
    Isometric,
}

#[derive(Component, Clone)]
#[require(TileStorage, Tileset, Name::new("Tilemap"), Transform, Visibility)]
pub struct TilemapLayer {
    pub chunks: HashMap<IVec2, Entity>,
    pub alpha_mode: AlphaMode2d,
    pub render_mode: TilemapRenderMode,
    pub z_index: i32,
}

impl Default for TilemapLayer {
    fn default() -> Self {
        Self {
            chunks: HashMap::new(),
            alpha_mode: AlphaMode2d::Blend,
            render_mode: TilemapRenderMode::default(),
            z_index: 0,
        }
    }
}

impl TilemapLayer {
    pub fn isometric() -> Self {
        Self {
            render_mode: TilemapRenderMode::Isometric,
            ..default()
        }
    }
}

/// Stores all tiles in a tilemap.
/// Maintains a mapping between tile positions and their entities.
#[derive(Component, Default, Reflect)]
#[relationship_target(relationship = TileOf)]
#[reflect(Component, Default)]
pub struct TilemapTiles(Vec<Entity>);

/// Marker component for tiles.
#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Tile;

/// Texture index for a tile.
/// The index corresponds to the position in the tilemap's texture atlas.
#[derive(Component, Clone, Debug, Default, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct TileTextureIndex(pub u16);

/// Position of a tile in the tilemap, in tile coordinates.
#[derive(Component, Clone, Copy, Debug, Default, Deref, DerefMut, Reflect, PartialEq, Eq, Hash)]
#[reflect(Component)]
#[require(OldTilePosition)]
pub struct TilePosition(pub IVec2);

impl TilePosition {
    /// Generates the four cardinal neighbors of this position,
    /// to the north, south, east, and west.
    pub fn cardinal_neighbors(&self) -> [TilePosition; 4] {
        [
            TilePosition(self.0 + IVec2::Y),
            TilePosition(self.0 - IVec2::Y),
            TilePosition(self.0 + IVec2::X),
            TilePosition(self.0 - IVec2::X),
        ]
    }
}

/// When a tile is moved, we need to keep track of its old position so we can
/// determine if it has moved to a new chunk.
#[derive(Component, Clone, Copy, Debug, Default, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct OldTilePosition(pub IVec2);

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
    // Transform,
    // Visibility
)]
#[relationship(relationship_target = TilemapTiles)]
#[reflect(Component)]
#[component(on_add = on_add_tile_of)]
pub struct TileOf(pub Entity);

fn on_add_tile_of(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let tilemap_entity = world.get::<TileOf>(entity).unwrap().0;
    let tile_position = *world.get::<TilePosition>(entity).unwrap();

    world.commands().entity(entity).insert((
        Name::new(format!("Tile {},{}", tile_position.x, tile_position.y)),
        ChildOf(tilemap_entity),
        TileDirty,
    ));
}

fn sync_tiles(
    mut commands: Commands,
    mut tiles_query: Query<
        (
            Entity,
            // &mut Transform,
            &TileOf,
            &TilePosition,
            &TileTextureIndex,
        ),
        (With<Tile>, With<TileDirty>),
    >,
    mut tile_storage_query: Query<(&mut TileStorage, &Tileset)>,
) {
    for (tile_entity, tile_of, tile_position, tile_texture_index) in &mut tiles_query {
        commands.entity(tile_entity).remove::<TileDirty>();
        let Ok((mut tile_storage, _tileset)) = tile_storage_query.get_mut(**tile_of) else {
            continue;
        };

        tile_storage.set(
            tile_position.0,
            Some(TileData::from_index(tile_texture_index.0)),
        );

        // transform.translation = (tile_position.as_vec2() * tileset.tile_size.as_vec2()).extend(0.0);
    }
}

/// A prelude module that re-exports all public items from the crate.
/// This is useful for importing all commonly used items at once.
pub mod prelude {
    pub use super::*;
}
