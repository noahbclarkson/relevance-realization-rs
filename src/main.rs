use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use relevance_realization_rs::plugins::{
    agent_plugin::AgentPlugin, app_state_plugin::AppStatePlugin, camera_plugin::CameraPlugin,
    new_world_plugin::NewWorldPlugin, tilemap_plugin::TileMapPlugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(EguiPlugin)
        .add_plugin(AppStatePlugin)
        .add_plugin(NewWorldPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(AgentPlugin)
        .run();
}
