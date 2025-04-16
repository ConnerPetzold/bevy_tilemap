# Bevy Tilemap

A Bevy plugin for efficient 2D tilemap rendering and management.

## Example

```rust
use bevy::prelude::*;
use bevy_tilemap::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((
            Tilemap {
                tile_size: 8,
                ..default()
            },
            TilemapTexture::Atlas(asset_server.load("tilemap.png")),
        ))
        .with_related_entities::<TileOf>(|t| {
            let map_size = Vec2::splat(32.0);
            let half_map = map_size / 2.0;
            for x in 0..map_size.x {
                for y in 0..map_size.y {
                    t.spawn((
                        TilePosition(Vec2::new(x, y) - half_map),
                        TileTextureIndex(0),
                    ));
                }
            }
        });

    commands.spawn(Camera2d);
}
```
