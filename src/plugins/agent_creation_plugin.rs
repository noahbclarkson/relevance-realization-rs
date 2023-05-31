// Game libraries and modules.
use super::{
    app_state_plugin::AppState,
    camera_plugin::{GameCamera, GameCameraPosition},
};
use crate::{
    agent::{
        agent::{Agent, AGENT_COUNT},
        agent_view::AgentView,
    },
    math::distance,
    tilemap::{Tiles, MAP_SIZE, TILE_SIZE},
};
use ::bevy::prelude::*;
use rand::Rng;
use std::collections::BTreeSet;

// This struct holds the camera lock targets and its state.
#[derive(Resource)]
pub struct CameraLockTarget {
    pub targets: Vec<Vec2>,
    pub locked: bool,
}

impl Default for CameraLockTarget {
    fn default() -> Self {
        CameraLockTarget {
            targets: Vec::new(),
            locked: false,
        }
    }
}

// Function to spawn agents in random locations on the map.
pub fn spawn_agents_in_random_locations(mut commands: Commands, asset_server: Res<AssetServer>) {
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

// Function to toggle camera lock based on middle mouse button input.
fn toggle_camera_lock_based_on_input(
    mouse_button_input: Res<Input<MouseButton>>,
    mut camera_lock_target: ResMut<CameraLockTarget>,
) {
    if mouse_button_input.just_pressed(MouseButton::Middle) {
        camera_lock_target.locked = !camera_lock_target.locked;
    }
}

// Function to get the view of each agent.
fn get_each_agents_view(mut query: Query<(&Transform, &mut Agent)>, tiles: Res<Tiles>) {
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

// Function to find all agents in the map.
fn find_all_agents(
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

// Function to lock camera to target.
fn lock_camera_to_selected_target(
    time: Res<Time>,
    camera_lock_target: ResMut<CameraLockTarget>,
    mut query: Query<(&mut Transform, &GameCamera)>,
    camera: Res<GameCameraPosition>,
) {
    if camera_lock_target.locked && !camera_lock_target.targets.is_empty() {
        let mut closest_screen_distance = f32::MAX;
        let mut target_pos = Vec2::ZERO;

        for target in camera_lock_target.targets.iter() {
            let screen_target = (*target - camera.pos) + (camera.window_size / 2.0);
            let screen_camera_position = (camera.pos - camera.pos) + (camera.window_size / 2.0);

            let screen_distance = distance(screen_camera_position, screen_target);

            if screen_distance < closest_screen_distance {
                closest_screen_distance = screen_distance;
                target_pos = *target;
            }
        }

        for (mut transform, _) in query.iter_mut() {
            let camera_pos = Vec2::new(transform.translation.x, transform.translation.y);
            let dist = distance(camera_pos, target_pos);

            if dist < 10.0 {
                continue;
            }

            let move_speed = 1.0 + dist / 100.0;

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

// Plugin to manage agent creation.
pub struct AgentCreationPlugin;

impl Plugin for AgentCreationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_agents_in_random_locations.in_schedule(OnEnter(AppState::InGame)))
            .insert_resource(CameraLockTarget::default())
            .add_system(toggle_camera_lock_based_on_input.in_set(OnUpdate(AppState::InGame)))
            .add_system(find_all_agents.in_set(OnUpdate(AppState::InGame)))
            .add_system(lock_camera_to_selected_target.in_set(OnUpdate(AppState::InGame)))
            .add_system(get_each_agents_view.in_set(OnUpdate(AppState::InGame)));
    }
}
