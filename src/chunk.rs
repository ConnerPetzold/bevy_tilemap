use bevy::{
    platform::collections::HashMap,
    prelude::*,
    sprite::{TilemapChunk, TilemapChunkIndices},
};

use crate::{
    Tile, TileDirty, TileOf, TilePosition, TilePositioning, TileTextureIndex, Tilemap,
    TilemapChunks, Tileset,
};

/// Stores the tilemap entity that the chunk belongs to
#[derive(Component, Clone, Debug, Reflect)]
#[require(TilemapChunkPosition, TilemapChunk)]
#[relationship(relationship_target = TilemapChunks)]
#[reflect(Component)]
pub struct TilemapChunkOf(pub Entity);

/// Position of the chunk in the tilemap in chunk coordinates (not pixel or tile coordinates)
#[derive(Component, Clone, Copy, Debug, Default, Deref, DerefMut, Reflect, PartialEq, Eq, Hash)]
#[reflect(Component)]
pub struct TilemapChunkPosition(pub IVec2);

/// Updates the chunks in the tilemap based on changes to tile positions.
/// This system is responsible for:
/// - Moving tiles between chunks when their positions change
/// - Creating new chunks when needed
/// - Updating chunk materials and textures
/// - Managing the parent-child relationships between tiles and chunks
pub(crate) fn update_chunks(
    mut commands: Commands,
    mut tiles_query: Query<
        (
            Entity,
            &mut Transform,
            &TileOf,
            &TilePosition,
            &TileTextureIndex,
        ),
        (With<Tile>, With<TileDirty>),
    >,
    mut tilemaps_query: Query<(&Tilemap, &mut TilemapChunks, &Tileset)>,
    mut chunks_query: Query<
        (Entity, &mut Transform, &mut TilemapChunkIndices),
        (With<TilemapChunk>, Without<Tile>),
    >,
) {
    let mut tiles_by_chunk: HashMap<(Entity, TilemapChunkPosition), Vec<Entity>> = HashMap::new();

    for (tile_entity, mut tile_transform, &TileOf(tilemap_entity), tile_position, _) in
        tiles_query.iter_mut()
    {
        let (tilemap, _, _) = tilemaps_query.get_mut(tilemap_entity).unwrap();

        let tilemap_chunk_position = tile_position.chunk_position(tilemap.chunk_size);

        tiles_by_chunk
            .entry((tilemap_entity, tilemap_chunk_position))
            .or_insert_with(Vec::new)
            .push(tile_entity);

        tile_transform.translation = tile_position
            .rem_euclid(IVec2::splat(tilemap.chunk_size as i32))
            .extend(0)
            .as_vec3();

        commands.entity(tile_entity).remove::<TileDirty>();
    }

    for ((tilemap_entity, tilemap_chunk_position), tiles) in tiles_by_chunk.iter() {
        let (tilemap, tilemap_chunks, Tileset(tileset_handle)) =
            tilemaps_query.get_mut(*tilemap_entity).unwrap();

        if let Some(chunk_entity) = tilemap_chunks.get(tilemap_chunk_position) {
            let (_, _, mut indices) = chunks_query.get_mut(*chunk_entity).unwrap();
            for (_, _, _, tile_position, tile_texture_index) in
                tiles.iter().map(|entity| tiles_query.get(*entity).unwrap())
            {
                indices[tile_position.index_in_chunk(tilemap.chunk_size)] =
                    Some(tile_texture_index.0);
            }

            commands.entity(*chunk_entity).add_children(tiles);
        } else {
            let mut indices: Vec<Option<u32>> =
                vec![None; (tilemap.chunk_size * tilemap.chunk_size) as usize];
            for (_, _, _, tile_position, tile_texture_index) in
                tiles.iter().map(|entity| tiles_query.get(*entity).unwrap())
            {
                indices[tile_position.index_in_chunk(tilemap.chunk_size)] =
                    Some(tile_texture_index.0);
            }

            let chunk_size_px = (tilemap.chunk_size * tilemap.tile_size) as i32;

            commands
                .spawn((
                    TilemapChunk {
                        chunk_size: UVec2::splat(tilemap.chunk_size),
                        tile_display_size: UVec2::splat(tilemap.tile_size),
                        tileset: tileset_handle.clone(),
                    },
                    TilemapChunkIndices(indices),
                    *tilemap_chunk_position,
                    TilemapChunkOf(*tilemap_entity),
                    ChildOf(*tilemap_entity),
                    Name::new(format!(
                        "TilemapChunk {},{}",
                        tilemap_chunk_position.x, tilemap_chunk_position.y
                    )),
                    Transform::from_xyz(
                        ((tilemap_chunk_position.x * chunk_size_px) + chunk_size_px / 2) as f32,
                        ((tilemap_chunk_position.y * chunk_size_px) + chunk_size_px / 2) as f32,
                        0.0,
                    ),
                ))
                .add_children(tiles);
        }
    }
}
