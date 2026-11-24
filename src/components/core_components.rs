use bevy::prelude::{default, AssetServer, Component, Sprite, Vec2};

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

// Markers
#[derive(Component)]
pub struct Movable;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Box;

#[derive(Component)]
pub struct BoxSpot;