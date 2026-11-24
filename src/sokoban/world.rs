use std::collections::HashMap;
use crate::sokoban::utils::{EntityId, Vector2D};

pub struct World {
    grid: HashMap<Vector2D, GridCell>,
}

#[derive(Debug)]
pub enum WorldError {
    CellNotFound(Vector2D),
    CellOccupied(Vector2D),
    EntityNotAtPosition(Vector2D)
}

#[derive(Clone, Copy)]
pub struct GridCell {
    pub tile: Tile,
    pub entity_id: Option<EntityId>,
}

impl GridCell {
    pub fn new(tile: Tile) -> Self {
        return Self {
            tile,
            entity_id: None
        };
    }
}

impl World {
    pub fn new() -> Self {
        return Self {
            grid: HashMap::new()
        };
    }

    pub fn get_grid(&self) -> &HashMap<Vector2D, GridCell> {
        return &self.grid;
    }

    pub fn add_tile(&mut self, position: Vector2D, tile: Tile) {
        self.grid.insert(position, GridCell::new(tile));
    }

    pub fn get_grid_cell(&self, position: Vector2D) -> Option<&GridCell> {
        return self.grid.get(&position);
    }

    pub fn add_entity(
        &mut self,
        entity_id: EntityId,
        position: Vector2D,
    ) -> Result<(), WorldError> {
        let cell = self.grid.get_mut(&position).ok_or(WorldError::CellNotFound(position))?;

        if cell.entity_id.is_some() {
            return Err(WorldError::CellOccupied(position));
        }

        cell.entity_id = Some(entity_id);
        return Ok(());
    }

    pub fn move_entity(&mut self, entity_id: EntityId,
                       old_position: Vector2D,
                       new_position: Vector2D) -> Result<(), WorldError> {

        let old_cell = self.grid
            .get(&old_position)
            .ok_or(WorldError::CellNotFound(old_position))?;

        let new_cell = self.grid
            .get(&new_position)
            .ok_or(WorldError::CellNotFound(new_position))?;

        if old_cell.entity_id != Some(entity_id) {
            return Err(WorldError::EntityNotAtPosition(old_position));
        }

        if new_cell.entity_id.is_some() {
            return Err(WorldError::CellOccupied(new_position));
        }

        self.grid.get_mut(&old_position).unwrap().entity_id = None;
        self.grid.get_mut(&new_position).unwrap().entity_id = Some(entity_id);

        return Ok(());
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorldEvent {
    EntityMoved {
        entity_id: u32,
        position: Vector2D
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Tile {
    Empty,
    Floor,
    Wall,
    BoxSpot
}