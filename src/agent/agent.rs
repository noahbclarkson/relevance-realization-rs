use ::bevy::prelude::*;
use getset::{Getters, Setters};

use crate::{
    agent::tradeoff::Tradeoff,
    positioning::{TilePosition, TransformPosition},
    tilemap::Tiles,
};

use super::task::{Task, TaskType};

pub const AGENT_COUNT: i32 = 10;
pub const AGENT_SPEED: f32 = 30.0;

#[derive(Component, Clone)]
pub struct Agent {
    pub task_history: Vec<Task>,
    pub data: AgentData,
    pub exploration_exploitation: Tradeoff,
    pub exploration_exploitation_state: ExploringExploitingState,
    pub latest_task: TaskType,
}

impl Agent {
    pub fn new() -> Self {
        Agent {
            task_history: Vec::new(),
            data: AgentData::default(),
            exploration_exploitation: Tradeoff::default(),
            exploration_exploitation_state: ExploringExploitingState::Exploring,
            latest_task: TaskType::Idle,
        }
    }

    pub fn agent_move(&mut self, time: &Res<Time>, task: &Task, transform: &mut Transform) {
        let mut agent_transform_position = TransformPosition::new_from_transform(transform);
        agent_transform_position.move_towards(&task.position().clone().into(), time, AGENT_SPEED);
        transform.translation = agent_transform_position.into_vec3(transform.translation.z);
    }

    pub fn exploit(&mut self, tiles: &Res<Tiles>, transform: &mut Transform) {
        let tile_position = TilePosition::new_from_transform(transform);
        let tile_type = tiles.get_tile_type(&tile_position);
        match tile_type {
            crate::tilemap::TileType::Grass => {
                self.data.saturation += 1.0;
                self.task_history
                    .push(Task::new(TaskType::Eat, tile_position, true));
            }
            crate::tilemap::TileType::Water => {
                self.data.thirst += 1.0;
                self.task_history
                    .push(Task::new(TaskType::Drink, tile_position, true));
            }
            crate::tilemap::TileType::DeepWater => {}
            crate::tilemap::TileType::Sand => {
                self.data.health += 1.0;
                self.task_history
                    .push(Task::new(TaskType::Regenerate, tile_position, true));
            }
            crate::tilemap::TileType::Mountain => {}
        }
        self.data.normalize();
    }

    pub fn find_latest_matching_task(&self, task_type: TaskType) -> Option<&Task> {
        for task in self.task_history.iter().rev() {
            if *task.task_type() == task_type && !task.completed() {
                return Some(task);
            }
        }
        None
    }

    pub fn check_if_at_postion(&self, transform: &Transform, position: &TilePosition) -> bool {
        let agent_position = TransformPosition::new_from_transform(transform);
        let tile_position = position.clone().into();
        agent_position.distance(&tile_position) < 1.0
    }

    pub fn tick(&mut self) {
        self.data.tick();
        self.data.normalize();
    }
}

impl std::fmt::Display for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // DIsplay the agents data line by line
        write!(f, "Saturation: {:.2}", self.data.saturation)?;
        write!(f, "\nThirst: {:.2}", self.data.thirst)?;
        write!(f, "\nHealth: {:.2}", self.data.health)?;
        write!(f, "\nState: {}", self.exploration_exploitation_state)?;
        write!(f, "\nCurrent Task: {}", self.latest_task)?;
        Ok(())
    }
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
        self.saturation -= 0.05;
        self.thirst -= 0.05;
        self.health -= 0.05;
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

#[derive(Component, Clone)]
pub enum ExploringExploitingState {
    Exploring,
    Exploiting,
}

impl std::fmt::Display for ExploringExploitingState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExploringExploitingState::Exploring => write!(f, "Exploring"),
            ExploringExploitingState::Exploiting => write!(f, "Exploiting"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::Transform;

    #[test]
    fn agent_new() {
        let agent = Agent::new();
        assert_eq!(agent.task_history.len(), 0);
        assert_eq!(agent.latest_task, TaskType::Idle);
    }

    #[test]
    fn agent_find_latest_matching_task() {
        let mut agent = Agent::new();
        let task = Task::new(TaskType::Eat, TilePosition::new(0, 0), false);
        agent.task_history.push(task.clone());
        assert_eq!(agent.find_latest_matching_task(TaskType::Eat), Some(&task));
    }

    #[test]
    fn agent_check_if_at_position() {
        let agent = Agent::new();
        let transform = Transform::default();
        let tile_position = TilePosition::new(0, 0);
        assert_eq!(agent.check_if_at_postion(&transform, &tile_position), true);
    }

    #[test]
    fn agent_data_default() {
        let agent_data = AgentData::default();
        assert_eq!(agent_data.saturation, 100.0);
        assert_eq!(agent_data.thirst, 100.0);
        assert_eq!(agent_data.health, 100.0);
    }

    #[test]
    fn agent_data_average() {
        let agent_data = AgentData::default();
        assert_eq!(agent_data.average(), 100.0);
    }

    #[test]
    fn agent_data_tick() {
        let mut agent_data = AgentData::default();
        agent_data.tick();
        assert_eq!(agent_data.saturation, 99.95);
        assert_eq!(agent_data.thirst, 99.95);
        assert_eq!(agent_data.health, 99.95);
    }

    #[test]
    fn agent_data_normalize() {
        let mut agent_data = AgentData::default();
        agent_data.saturation = 101.0;
        agent_data.normalize();
        assert_eq!(agent_data.saturation, 100.0);
    }

    #[test]
    fn agent_data_lowest() {
        let mut agent_data = AgentData::default();
        agent_data.saturation = 90.0;
        assert_eq!(agent_data.lowest(), 90.0);
    }

    #[test]
    fn agent_data_get_lowest_value_task() {
        let mut agent_data = AgentData::default();
        agent_data.saturation = 90.0;
        assert_eq!(agent_data.get_lowest_value_task(), TaskType::Eat);
    }
}

