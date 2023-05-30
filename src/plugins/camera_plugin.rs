use bevy::{input::Input, math::Vec3, prelude::*};

use crate::tilemap::{MAP_SIZE, TILE_SIZE};

use super::app_state_plugin::AppState;

const SPEED: f32 = 500.0;
const ZOOM_SPEED: f32 = 0.025;

// A simple camera system for moving and zooming the camera.
pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<GameCamera>>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Z) {
            ortho.scale += ZOOM_SPEED;
        }

        if keyboard_input.pressed(KeyCode::X) {
            ortho.scale -= ZOOM_SPEED;
        }

        if ortho.scale < 0.25 {
            ortho.scale = 0.25;
        }

        if ortho.scale > 1.5 {
            ortho.scale = 1.5;
        }

        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * SPEED;
        transform.translation.z = z;
    }
}

#[derive(Component)]
pub struct GameCamera;

fn setup(mut commands: Commands) {
    let pos = MAP_SIZE as f32 * TILE_SIZE as f32 / 2.0;
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(pos, pos, 1000.0),
            ..Default::default()
        },
        GameCamera,
    ));
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::InGame)))
            .add_system(movement.in_set(OnUpdate(AppState::InGame)));
    }
}
