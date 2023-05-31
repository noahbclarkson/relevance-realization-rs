use bevy::prelude::*;
use rand::rngs::ThreadRng;

use crate::{
    agent::{
        agent::{Agent, ExploringExploitingState},
        task::{Task, TaskType},
    },
    tilemap::Tiles,
};

use super::app_state_plugin::AppState;

fn calculate_agent_tasks(
    mut query: Query<(&mut Agent, &mut Transform)>,
    time: Res<Time>,
    tiles: Res<Tiles>,
) {
    let mut rng = &mut rand::thread_rng();
    for (mut agent, mut agent_transform) in query.iter_mut() {
        // Does this agent have a task history?
        if agent.task_history.is_empty() {
            let task = &mut Task::new(TaskType::Move, Tiles::random_position(&mut rng), false);
            agent.agent_move(&time, task, &mut agent_transform);
            agent.task_history.push(task.clone());
        } else {
            // Task frequency will be a number between 0 and 1 indicating how often the agent has executed the last task (weighted by recency).
            let lowest = agent.data.lowest() / 100.0;
            let lowest_negative = 1.0 - lowest;
            // Essentially, we want to explore if the agent is in a good state and we have repeated the same task a lot
            // We want to exploit if the agent is in a bad state and we have not repeated the same task a lot
            // We can average the two values to get a value between 0 and 1
            let exploration = lowest * agent.exploration_exploitation.lhs_multiplier();
            let exploitation = lowest_negative * agent.exploration_exploitation.rhs_multiplier();
            if exploration > exploitation {
                explore(&mut agent, &time, &tiles, &mut agent_transform, &mut rng);
            } else {
                let lowest_value_task_type = agent.data.get_lowest_value_task();
                // See if we have this task type in our task history, if we do, we can move towards it
                // If we don't we'll have to explore more
                let a = agent.clone();
                let task = a.find_latest_matching_task(lowest_value_task_type);
                if let Some(task) = task {
                    agent.agent_move(&time, task, &mut agent_transform);
                    let mut task = task.clone();
                    if agent.check_if_at_postion(&agent_transform, task.position()) {
                        task.set_completed(true);
                    }
                    agent.exploit(&tiles, &mut agent_transform);
                    agent.exploration_exploitation_state =
                        ExploringExploitingState::Exploiting;
                    agent.latest_task = lowest_value_task_type;
                } else {
                    explore(&mut agent, &time, &tiles, &mut agent_transform, &mut rng);
                }
            }
        }
    }
}

fn explore(
    agent: &mut Agent,
    time: &Res<Time>,
    tiles: &Res<Tiles>,
    mut agent_transform: &mut Transform,
    mut rng: &mut ThreadRng,
) {
    // To explore, we will find the latest move task that hasn't been completed
    // If we have completed all move tasks, we will generate another one at a random location
    let a = agent.clone();
    let task = a.find_latest_matching_task(TaskType::Move);
    if let Some(task) = task {
        agent.agent_move(&time, task, &mut agent_transform);
        let mut task = task.clone();
        if agent.check_if_at_postion(agent_transform, task.position()) {
            task.set_completed(true);
        }
        agent.task_history.push(task.clone());
        agent.latest_task = TaskType::Move;
    } else {
        let task = &mut Task::new(TaskType::Move, Tiles::random_position(&mut rng), false);
        agent.agent_move(&time, task, &mut agent_transform);
        let mut task = task.clone();
        if agent.check_if_at_postion(agent_transform, task.position()) {
            task.set_completed(true);
        }
        agent.task_history.push(task.clone());
        agent.latest_task = TaskType::Move;
    }
    // Then we want to exploit the current tile we are on
    agent.exploit(&tiles, &mut agent_transform);
    agent.exploration_exploitation_state = ExploringExploitingState::Exploring;

}

fn tick(mut agent_query: Query<&mut Agent>) {
    for mut agent in agent_query.iter_mut() {
        agent.tick();
    }
}

pub struct AgentBehaviourPlugin;

impl Plugin for AgentBehaviourPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(calculate_agent_tasks.in_set(OnUpdate(AppState::InGame)))
            .add_system(tick.in_set(OnUpdate(AppState::InGame)));
    }
}
