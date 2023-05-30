use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use relevance_realization_rs::{new_world::NewWorldPlugin, menu::AppStatePlugin, tilemap::TileMapPlugin, camera::CameraPlugin, agent::AgentPlugin};

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
