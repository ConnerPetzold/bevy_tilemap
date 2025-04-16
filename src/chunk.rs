use bevy::{
    asset::{RenderAssetUsages, weak_handle},
    platform_support::collections::HashMap,
    prelude::*,
    render::render_resource::{AsBindGroup, Extent3d, ShaderRef, TextureDimension, TextureFormat},
    sprite::Material2d,
};

use crate::{
    Tile, TileOf, TilePosition, TileTextureIndex, Tilemap, TilemapChunks, TilemapTexture,
    TilemapTiles,
};

/// Shader used for rendering tilemap chunks
pub const TILEMAP_CHUNK_MATERIAL_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("40f33e62-82f8-4578-b3fa-f22989e7c4bb");

/// Marker component for tilemap chunks.
#[derive(Component, Clone, Debug)]
#[require(TilemapChunkPosition, Transform, Visibility, Mesh2d, MeshMaterial2d<TilemapChunkMaterial>)]
pub struct TilemapChunk;

/// Position of the chunk in the tilemap in chunk coordinates (not pixel or tile coordinates)
#[derive(Component, Clone, Copy, Debug, Default, Deref, DerefMut, Reflect, PartialEq, Eq, Hash)]
#[reflect(Component)]
pub struct TilemapChunkPosition(pub IVec2);

/// Stores the tilemap entity that the chunk belongs to
#[derive(Component, Clone, Debug, Reflect)]
#[relationship(relationship_target = TilemapChunks)]
#[reflect(Component)]
pub struct TilemapChunkOf(pub Entity);

/// A material used for rendering tilemap chunks.
/// This material contains the necessary textures and uniforms for rendering
/// the tiles within a chunk.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TilemapChunkMaterial {
    /// The size of each tile in pixels
    #[uniform(0)]
    pub tile_size: u32,

    /// The texture atlas
    #[texture(1)]
    #[sampler(2)]
    pub atlas: Option<Handle<Image>>,

    /// The texture containing the indices of tiles within the chunk
    #[texture(3, sample_type = "u_int")]
    pub indices: Handle<Image>,
}

impl Material2d for TilemapChunkMaterial {
    fn fragment_shader() -> ShaderRef {
        TILEMAP_CHUNK_MATERIAL_SHADER_HANDLE.into()
    }
}

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
            Ref<TilePosition>,
            &TileTextureIndex,
        ),
        With<Tile>,
    >,
    mut tilemaps_query: Query<(&Tilemap, &mut TilemapChunks, &TilemapTexture), With<TilemapTiles>>,
    mut chunks_query: Query<
        (
            Entity,
            &mut Transform,
            &MeshMaterial2d<TilemapChunkMaterial>,
        ),
        (With<TilemapChunk>, Without<Tile>),
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TilemapChunkMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let mut tiles_by_chunk: HashMap<(Entity, TilemapChunkPosition), Vec<Entity>> = HashMap::new();

    for (tile_entity, mut tile_transform, &TileOf(tilemap_entity), tile_position, _) in
        tiles_query.iter_mut()
    {
        if !tile_position.is_changed() {
            continue;
        }

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
    }

    for ((tilemap_entity, tilemap_chunk_position), tiles) in tiles_by_chunk.iter() {
        let (tilemap, mut tilemap_chunks, TilemapTexture::Atlas(atlas)) =
            tilemaps_query.get_mut(*tilemap_entity).unwrap();

        let indices_image_handle = tilemap_chunks
            .get(tilemap_chunk_position)
            .and_then(|chunk_entity| chunks_query.get_mut(*chunk_entity).ok())
            .and_then(|(_, _, chunk_material)| materials.get_mut(chunk_material))
            .map(|chunk_material| chunk_material.indices.clone())
            .unwrap_or_else(|| {
                images.add(Image::new_fill(
                    Extent3d {
                        width: tilemap.chunk_size,
                        height: tilemap.chunk_size,
                        depth_or_array_layers: 1,
                    },
                    TextureDimension::D2,
                    &[0, 0, 0, 0],
                    TextureFormat::R32Uint,
                    RenderAssetUsages::default(),
                ))
            });

        let indices_image = images.get_mut(&indices_image_handle).unwrap();

        for (_, _, _, tile_position, tile_texture_index) in
            tiles.iter().map(|entity| tiles_query.get(*entity).unwrap())
        {
            let tile_position_in_chunk = tile_position
                .rem_euclid(IVec2::splat(tilemap.chunk_size as i32))
                .as_uvec2();

            indices_image
                .pixel_bytes_mut(uvec3(
                    tile_position_in_chunk.x,
                    tilemap.chunk_size - 1 - tile_position_in_chunk.y,
                    0,
                ))
                .unwrap()
                .copy_from_slice(&bytemuck::cast_slice(&[tile_texture_index.0 as u32 + 1]));
        }

        let tilemap_chunk_entity = tilemap_chunks
            .entry(*tilemap_chunk_position)
            .or_insert_with(|| {
                let chunk_size_px = (tilemap.chunk_size * tilemap.tile_size) as i32;

                commands
                    .spawn((
                        TilemapChunk,
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
                        Mesh2d(meshes.add(Rectangle::from_size(Vec2::splat(
                            (tilemap.chunk_size * tilemap.tile_size) as f32,
                        )))),
                        MeshMaterial2d(materials.add(TilemapChunkMaterial {
                            tile_size: tilemap.tile_size,
                            atlas: Some(atlas.clone()),
                            indices: indices_image_handle.clone(),
                        })),
                    ))
                    .id()
            });

        commands.entity(*tilemap_chunk_entity).add_children(tiles);
    }
}
