use bevy::prelude::{IVec2, Component};
use getset::{Getters, Setters};
use rand::Rng;

use crate::tilemap::MAP_SIZE;

#[derive(Component, Clone, Getters, Setters)]
pub struct Task {
    #[getset(get = "pub", set = "pub")]
    pub task_type: TaskType,
    #[getset(get = "pub", set = "pub")]
    pub location: IVec2,
}

impl Default for Task {
    fn default() -> Self {
        let mut rand = rand::thread_rng();
        Task {
            task_type: TaskType::Move,
            // A random tile on the map
            location: IVec2::new(rand.gen_range(0..MAP_SIZE), rand.gen_range(0..MAP_SIZE)),
        }
    }
}

impl Task {
    pub fn new(task_type: TaskType, location: IVec2) -> Self {
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