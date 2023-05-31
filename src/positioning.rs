use std::ops::{Add, Div, Mul, Sub};

use bevy::{
    prelude::{IVec2, Res, Transform, Vec2, Vec3},
    time::Time,
};

use crate::tilemap::TILE_SIZE;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}

impl TilePosition {
    pub fn new(x: i32, y: i32) -> Self {
        TilePosition { x, y }
    }

    pub fn new_from_transform(transform: &Transform) -> Self {
        TilePosition {
            x: (transform.translation.x / (TILE_SIZE as f32)).floor() as i32,
            y: (transform.translation.y / (TILE_SIZE as f32)).floor() as i32,
        }
    }
}

impl Default for TilePosition {
    fn default() -> Self {
        TilePosition { x: 0, y: 0 }
    }
}

impl From<IVec2> for TilePosition {
    fn from(vec: IVec2) -> Self {
        TilePosition { x: vec.x, y: vec.y }
    }
}

impl From<TilePosition> for IVec2 {
    fn from(tile_position: TilePosition) -> Self {
        IVec2::new(tile_position.x, tile_position.y)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransformPosition {
    pub x: f32,
    pub y: f32,
}

impl TransformPosition {
    pub fn new(x: f32, y: f32) -> Self {
        TransformPosition { x, y }
    }

    pub fn new_from_transform(transform: &Transform) -> Self {
        TransformPosition {
            x: transform.translation.x,
            y: transform.translation.y,
        }
    }

    pub fn distance(&self, other: &TransformPosition) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn normalized_direction(&self, other: &TransformPosition) -> TransformPosition {
        let distance = self.distance(other);
        TransformPosition {
            x: (other.x - self.x) / distance,
            y: (other.y - self.y) / distance,
        }
    }

    pub fn move_towards(&mut self, other: &TransformPosition, time: &Res<Time>, speed: f32) {
        let direction = self.normalized_direction(other);
        self.x += direction.x * time.delta_seconds() * speed;
        self.y += direction.y * time.delta_seconds() * speed;
    }

    pub fn into_vec3(&self, z: f32) -> Vec3 {
        Vec3::new(self.x, self.y, z)
    }
}

impl Default for TransformPosition {
    fn default() -> Self {
        TransformPosition { x: 0.0, y: 0.0 }
    }
}

impl From<TransformPosition> for (f32, f32) {
    fn from(transform_position: TransformPosition) -> Self {
        (transform_position.x, transform_position.y)
    }
}

impl From<TransformPosition> for IVec2 {
    fn from(transform_position: TransformPosition) -> Self {
        IVec2::new(transform_position.x as i32, transform_position.y as i32)
    }
}

impl From<IVec2> for TransformPosition {
    fn from(vec: IVec2) -> Self {
        TransformPosition {
            x: vec.x as f32,
            y: vec.y as f32,
        }
    }
}

impl From<TransformPosition> for TilePosition {
    fn from(transform_position: TransformPosition) -> Self {
        TilePosition {
            x: (transform_position.x / (TILE_SIZE as f32)).floor() as i32,
            y: (transform_position.y / (TILE_SIZE as f32)).floor() as i32,
        }
    }
}

impl From<TilePosition> for TransformPosition {
    fn from(tile_position: TilePosition) -> Self {
        TransformPosition {
            x: tile_position.x as f32 * TILE_SIZE as f32,
            y: tile_position.y as f32 * TILE_SIZE as f32,
        }
    }
}

impl From<TransformPosition> for Transform {
    fn from(transform_position: TransformPosition) -> Self {
        Transform::from_xyz(transform_position.x, transform_position.y, 0.0)
    }
}

impl From<Transform> for TransformPosition {
    fn from(transform: Transform) -> Self {
        TransformPosition::new_from_transform(&transform)
    }
}

impl From<TransformPosition> for Vec2 {
    fn from(transform_position: TransformPosition) -> Self {
        Vec2::new(transform_position.x, transform_position.y)
    }
}

impl From<Vec2> for TransformPosition {
    fn from(vec: Vec2) -> Self {
        TransformPosition { x: vec.x, y: vec.y }
    }
}

impl Sub for TransformPosition {
    type Output = TransformPosition;

    fn sub(self, rhs: Self) -> Self::Output {
        TransformPosition {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for TransformPosition {
    type Output = TransformPosition;

    fn add(self, rhs: Self) -> Self::Output {
        TransformPosition {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<f32> for TransformPosition {
    type Output = TransformPosition;

    fn mul(self, rhs: f32) -> Self::Output {
        TransformPosition {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for TransformPosition {
    type Output = TransformPosition;

    fn div(self, rhs: f32) -> Self::Output {
        TransformPosition {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Sub<Vec2> for TransformPosition {
    type Output = TransformPosition;

    fn sub(self, rhs: Vec2) -> Self::Output {
        TransformPosition {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add<Vec2> for TransformPosition {
    type Output = TransformPosition;

    fn add(self, rhs: Vec2) -> Self::Output {
        TransformPosition {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
