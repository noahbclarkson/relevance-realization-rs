use bevy::prelude::*;
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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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
    pub fn to_color(&self, rng: &mut ThreadRng) -> Color {
        let color = match self {
            TileType::Grass => Color::hex(GREEN).unwrap(),
            TileType::Water => Color::hex(BLUE).unwrap(),
            TileType::DeepWater => Color::hex(DARK_BLUE).unwrap(),
            TileType::Sand => Color::hex(YELLOW).unwrap(),
            TileType::Mountain => Color::hex(GRAY).unwrap(),
        };
        // Add some random variation to the color
        let extent = 0.0175;
        let r = rng.gen_range(-extent..extent);
        let g = rng.gen_range(-extent..extent);
        let b = rng.gen_range(-extent..extent);
        color + Color::rgb(r, g, b)
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
