use bevy::{prelude::*, render::color::HexColorError};
use rand::{rngs::ThreadRng, Rng};

use crate::positioning::TilePosition;

pub const MAP_SIZE: i32 = 250;
pub const TILE_SIZE: i32 = 16;

const GREEN: &str = "#40ff7c";
const BLUE: &str = "#2ab8f5";
const DARK_BLUE: &str = "#1e8bc3";
const YELLOW: &str = "#ffed69";
const GRAY: &str = "#6fa5bd";

#[derive(Component, PartialEq, Eq, Hash, Clone)]
pub struct Tile {
    pub position: TilePosition,
    pub tile_type: TileType,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum TileType {
    Grass,
    Water,
    DeepWater,
    Sand,
    Mountain,
}

impl Default for TileType {
    fn default() -> Self {
        TileType::Grass
    }
}

impl TileType {
    pub fn to_color(&self, rng: &mut ThreadRng) -> Result<Color, HexColorError> {
        let color = match self {
            TileType::Grass => Color::hex(GREEN)?,
            TileType::Water => Color::hex(BLUE)?,
            TileType::DeepWater => Color::hex(DARK_BLUE)?,
            TileType::Sand => Color::hex(YELLOW)?,
            TileType::Mountain => Color::hex(GRAY)?,
        };
        // Add some random variation to the color
        let extent = 0.0175;
        let r = rng.gen_range(-extent..extent);
        let g = rng.gen_range(-extent..extent);
        let b = rng.gen_range(-extent..extent);
        let color = color + Color::rgb(r, g, b);
        Ok(color)
    }
}

#[derive(Resource)]
pub struct Seed {
    pub tile_seed: u32,
    pub tile_entity_seed: u32,
}

impl Default for Seed {
    fn default() -> Self {
        Seed {
            tile_seed: rand::random(),
            tile_entity_seed: rand::random(),
        }
    }
}

#[derive(Resource)]
pub struct Tiles {
    pub tiles: [[TileType; MAP_SIZE as usize]; MAP_SIZE as usize],
}

impl Tiles {
    pub fn random_position(rng: &mut ThreadRng) -> TilePosition {
        let x = rng.gen_range(0..MAP_SIZE - 1);
        let y = rng.gen_range(0..MAP_SIZE - 1);
        TilePosition::new(x, y)
    }

    pub fn get_tile_type(&self, position: &TilePosition) -> TileType {
        self.tiles[position.x as usize][position.y as usize]
    }
}

impl Default for Tiles {
    fn default() -> Self {
        Tiles {
            tiles: [[TileType::default(); MAP_SIZE as usize]; MAP_SIZE as usize],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn tile_type_default() {
        let tile_type = TileType::default();
        assert_eq!(tile_type, TileType::Grass);
    }

    #[test]
    fn tile_type_to_color() {
        let mut rng = thread_rng();
        let color = TileType::Grass.to_color(&mut rng);
        assert!(color.is_ok());
    }

    #[test]
    fn tiles_random_position() {
        let mut rng = thread_rng();
        let position = Tiles::random_position(&mut rng);
        assert!(position.x >= 0);
        assert!(position.x < MAP_SIZE);
        assert!(position.y >= 0);
        assert!(position.y < MAP_SIZE);
    }

    #[test]
    fn tiles_get_tile_type() {
        let tiles = Tiles::default();
        let position = TilePosition::new(0, 0);
        assert_eq!(tiles.get_tile_type(&position), TileType::Grass);
    }

    #[test]
    fn tiles_default() {
        let tiles = Tiles::default();
        for i in 0..MAP_SIZE {
            for j in 0..MAP_SIZE {
                assert_eq!(tiles.tiles[i as usize][j as usize], TileType::Grass);
            }
        }
    }
}
