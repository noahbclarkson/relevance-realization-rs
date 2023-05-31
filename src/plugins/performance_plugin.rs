use bevy::{prelude::*, window::PrimaryWindow};

use crate::tilemap::TILE_SIZE;

use super::{app_state_plugin::AppState, camera_plugin::GameCamera, new_world_plugin::DEFAULT_SIDE_PANEL_WIDTH};

pub struct PerformancePlugin;

pub fn show_tiles_in_frame(
    mut query: Query<(&mut Visibility, &Transform), With<Sprite>>,
    camera_query: Query<(&Transform, &OrthographicProjection), With<GameCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let (camera_transform, ortho) = camera_query.single();
    let Ok(primary) = window_query.get_single() else {
        return;
    };
    let camera_x = camera_transform.translation.x;
    let camera_y = camera_transform.translation.y;

    let camera_left = camera_x - (primary.width() - DEFAULT_SIDE_PANEL_WIDTH + TILE_SIZE as f32 * 1.5) / 2.0 * (ortho.scale + 0.1);
    let camera_right = camera_x + (primary.width() + TILE_SIZE as f32 * 1.5) / 2.0 * (ortho.scale + 0.1);
    let camera_top = camera_y + (primary.height() + TILE_SIZE as f32 * 1.5) / 2.0 * (ortho.scale + 0.1);
    let camera_bottom = camera_y - (primary.height() + TILE_SIZE as f32 * 1.5) / 2.0 * (ortho.scale + 0.1);
    for (mut visibility, transform) in query.iter_mut() {
        let x = transform.translation.x;
        let y = transform.translation.y;
        if x > camera_left
            && x < camera_right
            && y > camera_bottom
            && y < camera_top
            && *visibility == Visibility::Hidden
        {
            *visibility = Visibility::Visible;
        } else if x < camera_left
            || x > camera_right
            || y < camera_bottom
            || y > camera_top && *visibility == Visibility::Visible
        {
            *visibility = Visibility::Hidden;
        }
    }
}

impl Plugin for PerformancePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(show_tiles_in_frame.in_set(OnUpdate(AppState::InGame)));
    }
}
