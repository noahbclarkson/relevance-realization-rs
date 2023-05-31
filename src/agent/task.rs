use bevy::prelude::Component;
use getset::{Getters, Setters};

use crate::positioning::TilePosition;

#[derive(Component, Clone, Debug, PartialEq, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Task {
    task_type: TaskType,
    position: TilePosition,
    completed: bool,
}

impl Task {
    pub fn new(task_type: TaskType, position: TilePosition, completed: bool) -> Self {
        Task {
            task_type,
            position,
            completed,
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

impl std::fmt::Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskType::Move => write!(f, "Move"),
            TaskType::Eat => write!(f, "Eat"),
            TaskType::Drink => write!(f, "Drink"),
            TaskType::Regenerate => write!(f, "Regenerate"),
            TaskType::Idle => write!(f, "Idle"),
        }
    }
}
