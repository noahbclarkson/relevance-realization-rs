use bevy::prelude::*;
use rand::Rng;

pub const MAP_SIZE: i32 = 250;
pub const TILE_SIZE: i32 = 16;

#[derive(Component, PartialEq, Eq, Hash, Clone)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
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
    pub fn to_color(&self) -> Color {
        let color = match self {
            TileType::Grass => Color::hex("#40ff7c").unwrap(),
            TileType::Water => Color::hex("#2ab8f5").unwrap(),
            TileType::DeepWater => Color::hex("#1e8bc3").unwrap(),
            TileType::Sand => Color::hex("#ffed69").unwrap(),
            TileType::Mountain => Color::hex("#6fa5bd").unwrap(),
        };
        // Add some random variation to the color
        let mut rng = rand::thread_rng();
        let extent = 0.015;
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
