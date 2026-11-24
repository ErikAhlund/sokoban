use std::sync::atomic::{AtomicU32, Ordering};

use crate::sokoban::utils::{WorldId, Vector2D};

static NEXT_ENTITY_ID: AtomicU32 = AtomicU32::new(0);

pub struct Entity {
    pub id: u32,
    pub world_id: WorldId,
    pub entity_type: EntityType,
    pub position: Vector2D
}

impl Entity {
    pub fn new(entity_type: EntityType, world_id: WorldId, position: Vector2D) -> Self {
        return Self {
            id: NEXT_ENTITY_ID.fetch_add(1, Ordering::Relaxed),
            entity_type,
            world_id,
            position
        };
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum EntityType {
    Player,
    Box,
    World {
        world_id: WorldId,
    },
}