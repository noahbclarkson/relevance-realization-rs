use bevy::prelude::Component;
use getset::{Getters, Setters};
use rand::Rng;

use crate::{positioning::TilePosition, tilemap::MAP_SIZE};

#[derive(Component, Clone, Getters, Setters)]
pub struct Task {
    #[getset(get = "pub", set = "pub")]
    pub task_type: TaskType,
    #[getset(get = "pub", set = "pub")]
    pub location: TilePosition,
}

impl Default for Task {
    fn default() -> Self {
        let mut rand = rand::thread_rng();
        Task {
            task_type: TaskType::Move,
            location: TilePosition::new(rand.gen_range(0..MAP_SIZE), rand.gen_range(0..MAP_SIZE)),
        }
    }
}

impl Task {
    pub fn new(task_type: TaskType, location: TilePosition) -> Self {
        Task {
            task_type,
            location,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TaskType {
    Move,
    Eat,
    Drink,
    Regenerate,
    Idle,
}
