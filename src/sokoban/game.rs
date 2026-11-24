use std::collections::HashMap;
use crate::sokoban::entity::Entity;
use crate::sokoban::{EntityType, GridCell};
use crate::sokoban::utils::{EntityId, Vector2D, WorldId};
use crate::sokoban::world::{Tile, World, WorldEvent};
use crate::sokoban::world::WorldEvent::EntityMoved;

pub struct Game {
    worlds: HashMap<WorldId, World>,
    entities: HashMap<EntityId, Entity>
}

impl Game {
    pub fn new() -> Self {
        return Self {
            worlds: HashMap::new(),
            entities: HashMap::new()
        }
    }

    pub fn get_world(&self, world_id: WorldId) -> &World {
        return self.worlds.get(&world_id).unwrap();
    }

    pub fn get_entity(&self, entity_id: EntityId) -> &Entity {
        return self.entities.get(&entity_id).unwrap();
    }

    pub fn load_maps(&mut self, maps: &[&str]) {
        for (index, map) in maps.iter().enumerate() {
            let world_id = index as u32;
            let mut world = World::new();

            let mut position = Vector2D::zero();
            for char in map.chars() {
                match char {
                    'W' => world.add_tile(position, Tile::Wall),
                    'S' => world.add_tile(position, Tile::BoxSpot),
                    'P' => {
                        world.add_tile(position, Tile::Floor);
                        let player = Entity::new(EntityType::Player, world_id, position);
                        world.add_entity(player.id, position).expect("Failed to add player");
                        self.entities.insert(player.id, player);
                    },
                    'B' => {
                        world.add_tile(position, Tile::Floor);
                        let box_entity = Entity::new(EntityType::Box, world_id, position);
                        world.add_entity(box_entity.id, position).expect("Failed to add box");
                        self.entities.insert(box_entity.id, box_entity);
                    }
                    'F' => {
                        world.add_tile(position, Tile::Floor);
                    }
                    '\n' => {
                        position.y += 1;
                        position.x = 0;
                        continue;
                    }
                    '0'..='9' => {
                        let index = char.to_digit(10).unwrap();
                        world.add_tile(position, Tile::Floor);
                        let world_cell = Entity::new(EntityType::World { world_id: index }, world_id, position);
                        world.add_entity(world_cell.id, position).expect("Failed to add box");
                        self.entities.insert(world_cell.id, world_cell);
                    }
                    _ => world.add_tile(position, Tile::Empty),
                };

                position.x += 1;
            }

            self.worlds.insert(world_id, world);
        }
    }

    pub fn apply_events(&mut self, events: &[WorldEvent]) {
        for event in events.iter() {
            match event {
                EntityMoved { entity_id, position } => {
                    self.move_entity(*entity_id, *position);
                }
            }
        }
    }

    pub fn move_entity(&mut self, entity_id: EntityId, position: Vector2D) {
        let entity = self.entities.get_mut(&entity_id).unwrap();
        let world = self.worlds.get_mut(&entity.world_id).unwrap();

        world.move_entity(entity_id, entity.position, position)
            .expect("World event contained an invalid entity movement");
        entity.position = position;
    }

    pub fn try_move_entity(&self, entity_id: u32, direction: Vector2D) -> Option<Vec<WorldEvent>>  {
        let entity = self.entities.get(&entity_id)?;
        let world =  self.worlds.get(&entity.world_id)?;

        let new_position = entity.position + direction;
        let grid_cell = world.get_grid_cell(new_position)?;

        if grid_cell.tile == Tile::Wall {
            return None;
        }

        let mut events = Vec::new();
        if let Some(blocking_entity) = grid_cell.entity_id {
            events = self.try_move_entity(blocking_entity, direction)?;
        }

        events.push(EntityMoved {
            entity_id,
            position: new_position,
        });

        return Some(events);
    }
}