use bevy::prelude::*;
use bevy_egui::{
    egui::{SidePanel, Slider, Ui},
    EguiContexts,
};

// Resource that holds the slider values
#[derive(Default, Resource)]
struct SliderValues {
    slider1: f32,
    slider2: f32,
    slider3: f32,
}

// Event that is sent when slider values are changed
struct SliderChangeEvent;

pub struct UserSettingsPlugin;

impl Plugin for UserSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SliderValues>()
            .add_event::<SliderChangeEvent>()
            .add_system(ui_system)
            .add_system(print_slider_values);
    }
}

// Helper function to create a slider and check if it was changed
fn add_slider(ui: &mut Ui, slider_value: &mut f32, label: &str) -> bool {
    ui.add(Slider::new(slider_value, 0.0..=100.0).text(label))
        .changed()
}

// System to create the UI and send SliderChangeEvent if any slider was changed
fn ui_system(
    mut contexts: EguiContexts,
    mut sliders: ResMut<SliderValues>,
    mut events: ResMut<Events<SliderChangeEvent>>,
) {
    SidePanel::left("side_panel")
        .default_width(200.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.heading("Settings");
                let mut changed = false;
                changed |= add_slider(ui, &mut sliders.slider1, "slider1");
                changed |= add_slider(ui, &mut sliders.slider2, "slider2");
                changed |= add_slider(ui, &mut sliders.slider3, "slider3");
                if changed {
                    events.send(SliderChangeEvent);
                }
            });
        });
}

// System to print the slider values when SliderChangeEvent is received
fn print_slider_values(mut events: EventReader<SliderChangeEvent>, sliders: Res<SliderValues>) {
    if !events.is_empty() {
        // Use info! instead of println! for better logging
        info!(
            "Slider values: {}, {}, {}",
            sliders.slider1, sliders.slider2, sliders.slider3
        );
        // Clear the events to avoid printing the same values multiple times
        events.clear();
    }
}
