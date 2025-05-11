# Bevy Tilemap

A Bevy plugin for efficient 2D tilemap rendering and management.

## Example

```rust
use bevy::{
    prelude::*,
    sprite::{Tilemap, Tileset},
};
use bevy_tilemap::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_size = IVec2::splat(1280);

    commands
        .spawn((
            Tilemap::default(),
            TilemapTiles::default(),
            Tileset {
                image: asset_server.load("atlas_packed.tileset.ron"),
                tile_size: UVec2::splat(8),
            },
        ))
        .with_related_entities::<TileOf>(|t| {
            for x in 0..map_size.x {
                for y in 0..map_size.y {
                    t.spawn((
                        TilePosition(ivec2(x - map_size.x / 2, y - map_size.y / 2)),
                        TileTextureIndex(rand::random_range(0..150)),
                    ));
                }
            }
        });

    commands.spawn(Camera2d);
}

```
