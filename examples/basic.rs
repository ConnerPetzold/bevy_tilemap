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
        .add_systems(Update, spawn_tilemap)
        .run();
}

#[derive(Resource)]
struct LoadingTilesetImage {
    is_loaded: bool,
    handle: Handle<TilesetImage>,
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(LoadingTilesetImage {
        is_loaded: false,
        handle: asset_server.load_with_settings::<TilesetImage, TilesetImageSettings>(
            "tilemap_packed.png",
            |settings| {
                settings.tile_size = UVec2::splat(8);
            },
        ),
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

fn spawn_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_tileset: ResMut<LoadingTilesetImage>,
    mut images: ResMut<Assets<Image>>,
    tileset_images: ResMut<Assets<TilesetImage>>,
) {
    if loading_tileset.is_loaded
        || !asset_server
            .load_state(loading_tileset.handle.id())
            .is_loaded()
    {
        return;
    }
    loading_tileset.is_loaded = true;

    let tileset_image = tileset_images.get(loading_tileset.handle.id()).unwrap();
    let image = images.add(tileset_image.0.clone());
    let map_size = IVec2::new(256, 256);

    commands
        .spawn((Tilemap::from_tile_size(8), Tileset(image)))
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
}
