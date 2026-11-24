mod hello_world;
mod components;
mod entities;
mod systems;
mod utils;
mod sokoban;

use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use crate::hello_world::HelloPlugin;
use crate::entities::*;
use crate::systems::update_positions;
use crate::utils::BG_COLOR;

fn main() {


        let mut game = sokoban::Game::new();
        let world_1 = "WWWWWWWW\nWFBFSFFW\nWFFF0FFW\nWFP0F1FW\nWFF1FFFW\nWFFF0FFW\nWWWWWWWW";
        let world_2 = "WWWWWWWW\nWFFFFFFW\nWF0F1FFW\nWFFF0FFW\nWFFFFFFW\nWFFFFFFW\nWWWWWWWW";

    game.load_maps(&[world_1, world_2]);

    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest()))
        .add_plugins(HelloPlugin)
        .insert_resource(GameResource(game))
        .insert_resource(ClearColor(BG_COLOR))
        .add_systems(Startup, create_map)
        .add_systems(Update, update_positions)
        .add_systems(Update, camera_controls)
        .run();

}

#[derive(Resource)]
struct GameResource(sokoban::Game);

fn create_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game: Res<GameResource>,
) {
    commands.spawn(Camera2d);

    commands.spawn((Transform::default(), Visibility::default())).with_children(|parent| {
        render_world(
            parent,
            &asset_server,
            &game.0,
            0,
            1,
        );
    });
}

use bevy::prelude::*;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bevy::prelude::*;

pub fn camera_controls(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut scroll: MessageReader<MouseWheel>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cameras: Query<
        (&Camera, &mut Transform, &mut Projection),
        With<Camera2d>,
    >,
) {
    let delta = time.delta_secs();

    let Ok(window) = windows.single() else {
        return;
    };

    let Ok((camera, mut transform, mut projection)) = cameras.single_mut() else {
        return;
    };

    // Movement
    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
        transform.translation += (direction * 500.0 * delta).extend(0.0);
    }

    // Zoom towards mouse cursor
    let scroll_amount: f32 = scroll.read().map(|event| event.y).sum();

    if scroll_amount == 0.0 {
        return;
    }

    let Some(cursor) = window.cursor_position() else {
        return;
    };

    let cursor_offset = cursor - window.size() / 2.0;

    if let Projection::Orthographic(ortho) = &mut *projection {
        let old_scale = ortho.scale;

        // Linear zoom, no acceleration
        let new_scale = (old_scale - scroll_amount * 0.01)
            .clamp(0.01, 100.0);

        // Convert cursor offset to world-space offset
        let world_offset = transform.rotation
            * Vec3::new(cursor_offset.x, -cursor_offset.y, 0.0);

        // Keep the point underneath the cursor stationary
        transform.translation += world_offset * (old_scale - new_scale);

        ortho.scale = new_scale;
    }
}