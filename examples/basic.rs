use bevy::{
    dev_tools::fps_overlay::FpsOverlayPlugin,
    prelude::*,
    remote::{RemotePlugin, http::RemoteHttpPlugin},
    sprite::{TilemapLayer, Tileset},
};
use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_tilemap::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Tilemap".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(FpsOverlayPlugin::default())
        .add_plugins(RemotePlugin::default())
        .add_plugins(RemoteHttpPlugin::default())
        .add_plugins(PanCamPlugin)
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_size = IVec2::splat(1280);

    commands
        .spawn((
            TilemapLayer::default(),
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

    commands.spawn((
        Camera2d,
        PanCam::default(),
        Projection::Orthographic(OrthographicProjection {
            scale: 0.25,
            ..OrthographicProjection::default_2d()
        }),
    ));
}
