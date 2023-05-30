use std::collections::BTreeSet;

use ::bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::{
    agent::{Agent, AgentView, AGENT_COUNT},
    math::distance,
    tilemap::{TileType, Tiles, MAP_SIZE, TILE_SIZE},
};

use super::{app_state_plugin::AppState, camera_plugin::GameCamera};

// Spawn the agent using player.png as the sprite.
pub fn spawn_agents(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the agent at a random spot on the map
    let mut rand = rand::thread_rng();
    let range = (MAP_SIZE * TILE_SIZE) - TILE_SIZE;
    for i in 0..AGENT_COUNT {
        let x = rand.gen_range(0..range);
        let y = rand.gen_range(0..range);
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("player.png"),
                transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                ..Default::default()
            })
            .insert(Agent::new());
        info!("Spawned agent {} at {}, {}", i, x, y);
    }
}

#[derive(Resource)]
pub struct CameraLockTarget {
    pub targets: Vec<Vec2>,
    pub camera_positon: Vec2,
    pub locked: bool,
    pub window_size: Vec2,
}

impl Default for CameraLockTarget {
    fn default() -> Self {
        CameraLockTarget {
            targets: Vec::new(),
            camera_positon: Vec2::ZERO,
            locked: false,
            window_size: Vec2::ZERO,
        }
    }
}

fn toggle_camera_lock(
    mouse_button_input: Res<Input<MouseButton>>,
    mut camera_lock_target: ResMut<CameraLockTarget>,
    query: Query<&Transform, With<GameCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(primary) = window_query.get_single() else {
        return;
    };
    if mouse_button_input.just_pressed(MouseButton::Middle) {
        camera_lock_target.locked = !camera_lock_target.locked;
        if camera_lock_target.locked {
            for transform in query.iter() {
                camera_lock_target.camera_positon =
                    Vec2::new(transform.translation.x, transform.translation.y);
            }
            camera_lock_target.window_size = Vec2::new(primary.width(), primary.height());
        }
    }
}

fn get_agent_view(mut query: Query<(&Transform, &mut Agent)>, tiles: Res<Tiles>) {
    for (transform, mut agent) in query.iter_mut() {
        let mut agent_view = BTreeSet::new();

        let agent_tile_position = IVec2::new(
            (transform.translation.x / TILE_SIZE as f32) as i32,
            (transform.translation.y / TILE_SIZE as f32) as i32,
        );

        let min_x = (agent_tile_position.x - agent.view_distance()).max(0);
        let max_x = (agent_tile_position.x + agent.view_distance()).min(MAP_SIZE as i32 - 1);
        let min_y = (agent_tile_position.y - agent.view_distance()).max(0);
        let max_y = (agent_tile_position.y + agent.view_distance()).min(MAP_SIZE as i32 - 1);

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let tile_position = IVec2::new(x, y);
                let tile_type = tiles.tiles[tile_position.x as usize][tile_position.y as usize];

                // Calculate the distance between the transform of the agent and the transform of the tile
                let tile_distance = distance(
                    Vec2::new(transform.translation.x, transform.translation.y),
                    Vec2::new(
                        (tile_position.x * TILE_SIZE) as f32,
                        (tile_position.y * TILE_SIZE) as f32,
                    ),
                );

                agent_view.insert(AgentView::new(tile_distance, tile_position, tile_type));
            }
        }

        agent.agent_view = agent_view;
    }
}

// Move the agent to the closest Sand tile
fn agent_move(time: Res<Time>, mut query: Query<(&mut Transform, &Agent)>) {
    for (mut transform, agent) in query.iter_mut() {
        // Check if agent view is not empty
        if agent.agent_view.is_empty() {
            continue;
        }

        // Find the closest sand tile
        let closest_sand_tile = agent
            .agent_view
            .iter()
            .filter(|view| *view.tile_type() == TileType::Sand)
            .next();

        // If a sand tile is found
        if let Some(closest_sand_tile) = closest_sand_tile {
            // Calculate target position in real coordinates (not tile coordinates)
            let target_x = (closest_sand_tile.tile_position().x * TILE_SIZE) as f32;
            let target_y = (closest_sand_tile.tile_position().y * TILE_SIZE) as f32;

            // Calculate the direction vector from the agent to the target position
            let direction_x = target_x - transform.translation.x;
            let direction_y = target_y - transform.translation.y;

            // Calculate the length of the direction vector
            let direction_length = (direction_x.powi(2) + direction_y.powi(2)).sqrt();

            let mut move_speed = 20.0;

            if direction_length < move_speed {
                move_speed = direction_length;
            }

            if direction_length > 0.0 {
                // Normalize the direction vector (make its length 1)
                let normalized_direction_x = direction_x / direction_length;
                let normalized_direction_y = direction_y / direction_length;

                // Update the agent's position, moving it in the direction of the target
                transform.translation.x +=
                    normalized_direction_x * time.delta_seconds() * move_speed;
                transform.translation.y +=
                    normalized_direction_y * time.delta_seconds() * move_speed;
            }
        }
    }
}

fn find_agents(
    mut camera_lock_target: ResMut<CameraLockTarget>,
    query: Query<(&Transform, &Agent)>,
) {
    camera_lock_target.targets.clear();
    for (transform, _) in query.iter() {
        camera_lock_target
            .targets
            .push(Vec2::new(transform.translation.x, transform.translation.y));
    }
}

fn lock_camera_to_target(
    time: Res<Time>,
    camera_lock_target: ResMut<CameraLockTarget>,
    mut query: Query<(&mut Transform, &GameCamera)>,
) {
    if camera_lock_target.locked && !camera_lock_target.targets.is_empty() {
        let mut closest_screen_distance = f32::MAX;
        let mut target_pos = Vec2::ZERO;

        for target in camera_lock_target.targets.iter() {
            let screen_target = (*target - camera_lock_target.camera_positon)
                + (camera_lock_target.window_size / 2.0);
            let screen_camera_position = (camera_lock_target.camera_positon
                - camera_lock_target.camera_positon)
                + (camera_lock_target.window_size / 2.0);

            let screen_distance = distance(screen_camera_position, screen_target);

            if screen_distance < closest_screen_distance {
                closest_screen_distance = screen_distance;
                target_pos = *target;
            }
        }

        for (mut transform, _) in query.iter_mut() {
            let camera_pos = Vec2::new(transform.translation.x, transform.translation.y);
            let dist = distance(camera_pos, target_pos);

            // If the distance is less than 5 don't move
            if dist < 10.0 {
                continue;
            }

            // Move faster if the distance is greater
            let move_speed = 1.0 + dist / 100.0;

            // Interpolate towards the target
            let x = transform.translation.x
                + (target_pos.x as f32 - transform.translation.x)
                    * time.delta_seconds()
                    * move_speed;
            let y = transform.translation.y
                + (target_pos.y as f32 - transform.translation.y) * time.delta_seconds() / 2.0
                    * move_speed;

            transform.translation = Vec3::new(x, y, transform.translation.z);
        }
    }
}

pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_agents.in_schedule(OnEnter(AppState::InGame)))
            .insert_resource(CameraLockTarget::default())
            .add_system(toggle_camera_lock.in_set(OnUpdate(AppState::InGame)))
            .add_system(find_agents.in_set(OnUpdate(AppState::InGame)))
            .add_system(lock_camera_to_target.in_set(OnUpdate(AppState::InGame)))
            .add_system(get_agent_view.in_set(OnUpdate(AppState::InGame)))
            .add_system(agent_move.in_set(OnUpdate(AppState::InGame)));
    }
}