use bevy::{
    dev_tools::fps_overlay::FpsOverlayPlugin,
    prelude::*,
    remote::{RemotePlugin, http::RemoteHttpPlugin},
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
        // .add_observer(on_add_tilemap_chunk)
        .run();
}

fn startup(mut commands: Commands, assets: Res<AssetServer>) {
    let map_size = IVec2::splat(1280);

    let tileset = Tileset {
        image: assets.load("isometric/atlas.tileset.ron"),
        tile_size: UVec2::splat(32),
    };

    commands
        .spawn((
            TilemapLayer {
                render_mode: TilemapRenderMode::Isometric,
                z_index: 0,
                ..default()
            },
            TilemapTiles::default(),
            tileset.clone(),
        ))
        .with_related_entities::<TileOf>(|t| {
            for x in 0..map_size.x {
                for y in 0..map_size.y {
                    t.spawn((
                        TilePosition(ivec2(x - map_size.x / 2, y - map_size.y / 2)),
                        TileTextureIndex(rand::random_range(0..11)),
                    ));
                }
            }
        });

    commands
        .spawn((
            TilemapLayer {
                render_mode: TilemapRenderMode::Isometric,
                z_index: 1,
                ..default()
            },
            TilemapTiles::default(),
            tileset,
            Transform::from_xyz(0.0, 8.0, 0.0),
        ))
        .with_related_entities::<TileOf>(|t| {
            for x in 0..map_size.x {
                for y in 0..map_size.y {
                    if rand::random_bool(0.3) {
                        t.spawn((
                            TilePosition(ivec2(x - map_size.x / 2, y - map_size.y / 2)),
                            TileTextureIndex(rand::random_range(0..11)),
                        ));
                    }
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

fn on_add_tilemap_chunk(trigger: Trigger<OnAdd, TilemapChunk>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert(ShowAabbGizmo::default());
}
