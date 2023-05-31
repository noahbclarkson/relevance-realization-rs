use getset::{Getters, Setters};
use ordered_float::NotNan;

use crate::{positioning::TilePosition, tilemap::TileType};

// Struct to represent the agent's view
#[derive(Clone, Getters, Setters)]
pub struct AgentView {
    #[getset(get = "pub")]
    distance: f32,
    #[getset(get = "pub")]
    tile_position: TilePosition,
    #[getset(get = "pub")]
    tile_type: TileType,
}

// Implementation of comparison traits for AgentView, using NotNan to handle potential NaN values in tile_distance
impl PartialOrd for AgentView {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        NotNan::new(self.distance)
            .unwrap()
            .partial_cmp(&NotNan::new(other.distance).unwrap())
    }
}

impl Ord for AgentView {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        NotNan::new(self.distance)
            .unwrap()
            .cmp(&NotNan::new(other.distance).unwrap())
    }
}

impl PartialEq for AgentView {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for AgentView {}

// Default implementation for AgentView
impl Default for AgentView {
    fn default() -> Self {
        AgentView {
            distance: f32::MAX,
            tile_position: TilePosition::default(),
            tile_type: TileType::default(),
        }
    }
}

impl AgentView {
    pub fn new(distance: f32, tile_position: TilePosition, tile_type: TileType) -> Self {
        AgentView {
            distance,
            tile_position,
            tile_type,
        }
    }
}
