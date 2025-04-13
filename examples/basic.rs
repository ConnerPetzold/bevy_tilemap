use bevy::{
    prelude::*,
    remote::{RemotePlugin, http::RemoteHttpPlugin},
};
use bevy_tilemap::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RemotePlugin::default())
        .add_plugins(RemoteHttpPlugin::default())
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_size = IVec2::new(32, 32);

    commands
        .spawn((
            Tilemap::default(),
            TilemapTexture::Atlas(asset_server.load("tilemap_packed.png")),
        ))
        .with_related_entities::<TileOf>(|t| {
            for x in 0..map_size.x {
                for y in 0..map_size.y {
                    t.spawn(TilePosition(ivec2(x, y)));
                }
            }
        });

    commands.spawn(Camera2d);
}
