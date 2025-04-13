use bevy::{
    asset::weak_handle,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::{Tile, TileOf, TilePosition, Tilemap, TilemapChunks, TilemapTiles};

pub const TILEMAP_CHUNK_MATERIAL_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("40f33e62-82f8-4578-b3fa-f22989e7c4bb");

#[derive(Component, Clone, Debug)]
#[require(TilemapChunkPosition, Transform, Visibility, Mesh2d, MeshMaterial2d<TilemapChunkMaterial>)]
pub struct TilemapChunk;

#[derive(Component, Clone, Copy, Debug, Default, Deref, DerefMut, Reflect, PartialEq, Eq, Hash)]
#[reflect(Component)]
pub struct TilemapChunkPosition(pub IVec2);

#[derive(Component, Clone, Debug, Reflect)]
#[relationship(relationship_target = TilemapChunks)]
#[reflect(Component)]
pub struct TilemapChunkOf(pub Entity);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TilemapChunkMaterial {
    #[uniform(0)]
    pub tile_size: u32,

    #[texture(1)]
    #[sampler(2)]
    pub atlas: Option<Handle<Image>>,

    #[texture(3, sample_type = "u_int")]
    pub indices: Handle<Image>,
}

impl Material2d for TilemapChunkMaterial {
    fn fragment_shader() -> ShaderRef {
        TILEMAP_CHUNK_MATERIAL_SHADER_HANDLE.into()
    }
}

pub(crate) fn update_chunks(
    mut commands: Commands,
    mut tiles_query: Query<(Entity, &mut Transform, &TileOf, Ref<TilePosition>), With<Tile>>,
    mut tilemaps_query: Query<(&Tilemap, &mut TilemapChunks), With<TilemapTiles>>,
    // mut chunks_query: Query<(Entity, &mut Transform), (With<TilemapChunk>, Without<Tile>)>,
) {
    for (tile_entity, mut tile_transform, &TileOf(tilemap_entity), tile_position) in
        tiles_query.iter_mut()
    {
        if !tile_position.is_changed() {
            continue;
        }

        let (tilemap, mut tilemap_chunks) = tilemaps_query.get_mut(tilemap_entity).unwrap();

        let tilemap_chunk_position = tile_position.chunk_position(tilemap.tile_size);

        let tilemap_chunk_entity =
            tilemap_chunks
                .entry(tilemap_chunk_position)
                .or_insert_with(|| {
                    commands
                        .spawn((
                            TilemapChunk,
                            tilemap_chunk_position,
                            TilemapChunkOf(tilemap_entity),
                            ChildOf(tilemap_entity),
                            Name::new(format!(
                                "TilemapChunk {},{}",
                                tilemap_chunk_position.x, tilemap_chunk_position.y
                            )),
                        ))
                        .id()
                });

        commands
            .entity(tile_entity)
            .insert(ChildOf(*tilemap_chunk_entity));

        tile_transform.translation = tile_position
            .rem_euclid(IVec2::splat(tilemap.tile_size as i32))
            .extend(0)
            .as_vec3();
    }
}
