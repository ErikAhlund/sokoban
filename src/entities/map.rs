use bevy::prelude::*;
use crate::components::*;
use crate::utils::TILE_SIZE;

use crate::{components, sokoban};

pub fn render_world(parent: &mut ChildSpawnerCommands, asset_server: &Res<AssetServer>,
                    game: &sokoban::Game, world_id: sokoban::WorldId, depth: u8) {

    if depth > 4 {
        return;
    }

    let world = game.get_world(world_id);
    let grid = world.get_grid();
    for (position, grid_cell) in grid.iter() {
        let position = GridPosition {
            x: position.x-3,
            y: position.y-3,
            z: 10*depth as i32
        };

        println!{"Rendering at position {:?}", position};

        create_sokoban_tile(parent, asset_server, grid_cell.tile, position);

        if grid_cell.entity_id.is_some() {
            let entity = game.get_entity(grid_cell.entity_id.unwrap());
            create_sokoban_entity(parent, asset_server, entity.entity_type, position, game, depth)
        }

    }
}

fn create_sokoban_tile(parent: &mut ChildSpawnerCommands, asset_server: &Res<AssetServer>,
                       tile: sokoban::Tile, position: GridPosition) {
    println!{"We are creating a {:?} tile ", tile}
    match tile {
        sokoban::Tile::Empty => {}
        sokoban::Tile::Floor => create_floor(parent, asset_server, position),
        sokoban::Tile::Wall => create_wall(parent, asset_server, position),
        sokoban::Tile::BoxSpot => {
            create_floor(parent, asset_server, position);
            create_box_spot(parent, asset_server, position);
        },
    }
}

fn create_sokoban_entity(parent: &mut ChildSpawnerCommands, asset_server: &Res<AssetServer>,
                       entity_type: sokoban::EntityType, position: GridPosition, game: &sokoban::Game, depth: u8) {

    println!{"We are creating a {:?} entity ", entity_type}
    match entity_type {
        sokoban::EntityType::Player => create_player(parent, asset_server, position),
        sokoban::EntityType::Box => create_box(parent, asset_server, position),
        sokoban::EntityType::World { world_id } => { create_world(parent, asset_server, game, world_id, position, depth+1) }
        _ => {println!{"Skipped entity!"}}
    }
}

fn create_world(parent: &mut ChildSpawnerCommands, asset_server: &Res<AssetServer>,
                game: &sokoban::Game, world_id: sokoban::WorldId, position: GridPosition, depth: u8) {

    let mut transform = Transform::default();
    transform.scale = Vec3::splat(0.1);
    parent.spawn((position, transform, Visibility::default())).with_children(|child| {
        render_world(
            child,
            &asset_server,
            &game,
            world_id,
            depth,
        );
    });
}

pub fn create_wall(parent: &mut ChildSpawnerCommands, asset_server: &Res<AssetServer>, mut position: GridPosition) {
    position.z += 10;
    parent.spawn((
        position,
        create_sprite(asset_server, "sprites/wall.png"),
        Wall {},
    ));
}

pub fn create_floor(parent: &mut ChildSpawnerCommands, asset_server: &Res<AssetServer>, mut position: GridPosition) {
    position.z += 1;
    parent.spawn((
        position,
        create_sprite(asset_server, "sprites/floor.png"),
    ));
}

pub fn create_box_spot(parent: &mut ChildSpawnerCommands, asset_server: &Res<AssetServer>, mut position: GridPosition) {
    position.z += 2;
    parent.spawn((
        position,
        create_sprite(asset_server, "sprites/box_spot.png"),
    ));
}

pub fn create_box(parent: &mut ChildSpawnerCommands, asset_server: &Res<AssetServer>, mut position: GridPosition) {
    position.z += 5;
    parent.spawn((
        position,
        create_sprite(asset_server, "sprites/box.png"),
        components::Box {},
    ));
}

pub fn create_player(parent: &mut ChildSpawnerCommands, asset_server: &Res<AssetServer>, mut position: GridPosition) {
    position.z += 5;
    parent.spawn((
        position,
        create_sprite(asset_server, "sprites/player.png"),
        Player {},
    ));
}

fn create_sprite(asset_server: &Res<AssetServer>, path: &str) -> Sprite {
    Sprite {
        image: asset_server.load(path.to_owned()),
        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
        ..default()
    }
}