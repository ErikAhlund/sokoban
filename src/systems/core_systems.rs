use bevy::math::Vec3;
use bevy::prelude::{Changed, Query, Transform};
use crate::components::GridPosition;
use crate::utils::{TILE_SIZE};

pub fn update_positions(
    mut query: Query<
        (&GridPosition, &mut Transform),
        Changed<GridPosition>,
    >,
) {
    for (position, mut transform) in &mut query {
        transform.translation.x = position.x as f32 * TILE_SIZE;
        transform.translation.y = -(position.y as f32) * TILE_SIZE;
        transform.translation.z = position.z as f32;
    }
}