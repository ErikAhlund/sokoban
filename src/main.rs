use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest()))
        .add_systems(Startup, create_map)
        .run();
}

fn create_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(Sprite {
        image: asset_server.load("sprites/wall.png"),
        custom_size: Some(Vec2::new(100., 100.)),
        ..default()
    });
}

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Box;

#[derive(Component)]
pub struct BoxSpot;