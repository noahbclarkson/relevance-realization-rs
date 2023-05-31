use std::collections::BTreeSet;

use ::bevy::prelude::*;
use getset::{Getters, Setters};
use ordered_float::NotNan;
use rand::Rng;

use crate::{tilemap::{TileType, MAP_SIZE}, tradeoff::{Tradeoff}};

pub const AGENT_COUNT: i32 = 10;
pub const AGENT_VIEW_DISTANCE: i32 = 32;

#[derive(Component, Clone, Getters, Setters)]
pub struct Agent {
    pub agent_view: BTreeSet<AgentView>,
    pub task_history: Vec<Task>,
    pub data: AgentData,
    pub exploration_exploitation: Tradeoff,
    #[getset(get = "pub", set = "pub")]
    view_distance: i32,
}

impl Agent {
    pub fn new() -> Self {
        Agent {
            agent_view: BTreeSet::new(),
            task_history: Vec::new(),
            data: AgentData::default(),
            exploration_exploitation: Tradeoff::default(),
            view_distance: AGENT_VIEW_DISTANCE / 2,
        }
    }

    pub fn calculate_exploration_exploitation(&mut self) -> (f32, f32) {
        let binding = Task::default();
        let last_task = self.task_history.last().unwrap_or(&binding);
        let mut last_100_tasks = Vec::new();
        for (i, task) in self.task_history.iter().rev().enumerate() {
            if i == 100 {
                break;
            }
            last_100_tasks.push(task);
        }
        let mut same_task_count = 0;
        for task in last_100_tasks.iter() {
            if task.task_type == last_task.task_type {
                same_task_count += 1;
            }
        }
        let same_task_percentage = same_task_count as f32 / 100.0;
        let exploration = same_task_percentage * self.exploration_exploitation.lhs_multiplier();
        let exploitation = self.data.lowest() / 100.0 * self.exploration_exploitation.rhs_multiplier();
        (exploration, exploitation)
    }
}

impl std::fmt::Display for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Display the view distance
        write!(f, "View distance: {}", self.view_distance)?;
        // DIsplay the agents data line by line
        write!(f, "\nData:")?;
        write!(f, "\n\tSaturation: {}", self.data.saturation)?;
        write!(f, "\n\tThirst: {}", self.data.thirst)?;
        write!(f, "\n\tHealth: {}", self.data.health)?;
        Ok(())
    }
}

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

#[derive(Component, Clone, Getters, Setters)]
pub struct AgentData {
    pub saturation: f32,
    pub thirst: f32,
    pub health: f32,
}

impl Default for AgentData {
    fn default() -> Self {
        AgentData {
            saturation: 100.0,
            thirst: 100.0,
            health: 100.0,
        }
    }
}

impl AgentData {
    pub fn average(&self) -> f32 {
        (self.saturation + self.thirst + self.health) / 3.0
    }

    pub fn tick(&mut self) {
        self.saturation -= 0.01;
        self.thirst -= 0.01;
        self.health -= 0.01;
        if (self.saturation < 0.0) || (self.thirst < 0.0) || (self.health < 0.0) {
            self.health = 0.0;
        }
    }

    pub fn normalize(&mut self) {
        // If any of the values are above 100 set them to 100
        if self.saturation > 100.0 {
            self.saturation = 100.0;
        }
        if self.thirst > 100.0 {
            self.thirst = 100.0;
        }
        if self.health > 100.0 {
            self.health = 100.0;
        }
    }

    pub fn lowest(&self) -> f32 {
        self.saturation.min(self.thirst.min(self.health))
    }

    pub fn get_lowest_value_task(&self) -> TaskType {
        if self.saturation < self.thirst {
            if self.saturation < self.health {
                TaskType::Eat
            } else {
                TaskType::Regenerate
            }
        } else {
            if self.thirst < self.health {
                TaskType::Drink
            } else {
                TaskType::Regenerate
            }
        }
    }
}
