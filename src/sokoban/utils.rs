use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Debug)]
pub struct Vector2D {
    pub x: i32,
    pub y: i32,
}

impl Vector2D {
    pub fn new(x: i32, y: i32) -> Self {
        return Self {
            x, y
        };
    }

    pub fn zero() -> Self {
        return Self::new(0, 0);
    }
}

impl Add for Vector2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

pub type EntityId = u32;
pub type WorldId = u32;