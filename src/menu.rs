use bevy::prelude::*;
use bevy_egui::{
    egui::{self, CentralPanel},
    EguiContexts, EguiSettings,
};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .insert_resource(Msaa::Sample4)
            .add_system(menu.in_set(OnUpdate(AppState::Menu)));
    }
}

fn menu(mut contexts: EguiContexts, mut state: ResMut<NextState<AppState>>, mut egui_settings: ResMut<EguiSettings>) {
    let ctx = contexts.ctx_mut();
    egui_settings.scale_factor = 2.3;
    CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.heading("Relevance Realization Menu");
            ui.add(egui::Hyperlink::from_label_and_url(
                "Source Code",
                "https://github.com/noahbclarkson/relevance-realization-rs",
            ));
            
            ui.separator();

            if ui.add(egui::Button::new("Start").min_size(egui::Vec2 {x: 80.0, y: 20.0})).clicked() {
                state.set(AppState::InGame);
                egui_settings.scale_factor = 1.0;
            }
            if ui.add(egui::Button::new("Quit").fill(egui::Color32::BLACK).min_size(egui::Vec2 {x: 80.0, y: 20.0})).clicked() {
                std::process::exit(0);
            }

        });
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            egui::warn_if_debug_build(ui);
        });
    });
    
}
