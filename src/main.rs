use bevy::prelude::*;
use relevance_realization_rs::user_settings::UserSettingsPlugin;

fn main() {
    App::new()
        // This will enable the logging system
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_egui::EguiPlugin)
        .add_plugin(UserSettingsPlugin)
        .add_startup_system(setup)
        .run();
}

// This system initializes the Camera2dBundle
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
