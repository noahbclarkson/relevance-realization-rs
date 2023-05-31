use bevy::prelude::IVec2;
use getset::{Getters, Setters};
use ordered_float::NotNan;

use crate::tilemap::TileType;

// Struct to represent the agent's view
#[derive(Clone, Getters, Setters)]
pub struct AgentView {
    #[getset(get = "pub")]
    tile_distance: f32,
    #[getset(get = "pub")]
    tile_position: IVec2,
    #[getset(get = "pub")]
    tile_type: TileType,
}

// Implementation of comparison traits for AgentView, using NotNan to handle potential NaN values in tile_distance
impl PartialOrd for AgentView {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        NotNan::new(self.tile_distance)
            .unwrap()
            .partial_cmp(&NotNan::new(other.tile_distance).unwrap())
    }
}

impl Ord for AgentView {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        NotNan::new(self.tile_distance)
            .unwrap()
            .cmp(&NotNan::new(other.tile_distance).unwrap())
    }
}

impl PartialEq for AgentView {
    fn eq(&self, other: &Self) -> bool {
        self.tile_distance == other.tile_distance
    }
}

impl Eq for AgentView {}

// Default implementation for AgentView
impl Default for AgentView {
    fn default() -> Self {
        AgentView {
            tile_distance: f32::MAX,
            tile_position: IVec2::ZERO,
            tile_type: TileType::default(),
        }
    }
}

impl AgentView {
    pub fn new(tile_distance: f32, tile_position: IVec2, tile_type: TileType) -> Self {
        AgentView {
            tile_distance,
            tile_position,
            tile_type,
        }
    }
}