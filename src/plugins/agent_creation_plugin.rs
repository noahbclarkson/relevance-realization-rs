// Game libraries and modules.
use super::{
    agent_behaviour_plugin::AgentBehaviourPlugin,
    app_state_plugin::AppState,
    camera_plugin::{GameCamera, GameCameraPosition},
};
use crate::{
    agent::agent::{Agent, AGENT_COUNT},
    positioning::TransformPosition,
    tilemap::{MAP_SIZE, TILE_SIZE},
};
use ::bevy::prelude::*;
use rand::Rng;

// This struct holds the camera lock targets and its state.
#[derive(Resource)]
pub struct CameraLockTarget {
    pub targets: Vec<TransformPosition>,
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
    input: Res<Input<KeyCode>>,
    mut camera_lock_target: ResMut<CameraLockTarget>,
) {
    if input.just_pressed(KeyCode::E) {
        camera_lock_target.locked = !camera_lock_target.locked;
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
            .push(TransformPosition::new_from_transform(transform));
    }
}

// Function to lock camera to target.
fn lock_camera_to_selected_target(
    time: Res<Time>,
    camera_lock_target: ResMut<CameraLockTarget>,
    camera: Res<GameCameraPosition>,
    mut query: Query<&mut Transform, With<GameCamera>>,
) {
    if camera_lock_target.locked && !camera_lock_target.targets.is_empty() {
        let mut closest_screen_distance = f32::MAX;
        let mut target_pos = TransformPosition::default();

        for target in camera_lock_target.targets.iter() {
            let screen_target = (*target - camera.pos) + (camera.window_size / 2.0);
            let screen_camera_position = (camera.pos - camera.pos) + (camera.window_size / 2.0);

            let screen_distance = screen_camera_position.distance(&screen_target);

            if screen_distance < closest_screen_distance {
                closest_screen_distance = screen_distance;
                target_pos = *target;
            }
        }

        for mut transform in query.iter_mut() {
            let mut camera_pos = TransformPosition::new_from_transform(&transform);
            let dist = camera_pos.distance(&target_pos);

            let speed = dist * 2.0;
            camera_pos.move_towards(&target_pos, &time, speed);
            transform.translation = camera_pos.into_vec3(transform.translation.z);
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
            .add_plugin(AgentBehaviourPlugin);
    }
}
