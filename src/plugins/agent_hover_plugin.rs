use bevy::{prelude::*, window::PrimaryWindow};

use crate::{agent::agent::Agent, math::distance};

use super::{app_state_plugin::AppState, camera_plugin::GameCameraPosition};

// Radius within which an agent can be detected.
const AGENT_DETECTION_RADIUS: f32 = 16.0;

// Represents an agent that is currently being hovered over.
#[derive(Default, Resource, Clone)]
pub struct CurrentlyHoveredAgent {
    pub entity: Option<Entity>,
    pub position: Vec2,
}

// UI Component for displaying agent information.
#[derive(Component)]
pub struct AgentInfoDisplayUI;

// Function to detect which agent is currently being hovered over.
fn detect_currently_hovered_agent(
    mut hovered_agent: ResMut<CurrentlyHoveredAgent>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut agent_query: Query<(Entity, &Transform), With<Agent>>,
    orthographic_projection_query: Query<&OrthographicProjection>,
    camera: Res<GameCameraPosition>,
) {
    if let Ok(primary_window) = window_query.get_single() {
        if let Some(cursor_position) = primary_window.cursor_position() {
            // Initialize with no agent being hovered over.
            hovered_agent.entity = None;

            // Obtain the orthographic projection of the camera.
            let ortho_projection = orthographic_projection_query.single();

            // Calculate camera's position considering the window's dimensions.
            let camera_position = Vec2::new(
                camera.pos.x - (primary_window.width() / 2.0) * ortho_projection.scale,
                camera.pos.y - (primary_window.height() / 2.0) * ortho_projection.scale,
            );

            // Calculate the cursor's position considering the scale.
            let cursor_position_scaled = Vec2::new(
                cursor_position.x as f32 * ortho_projection.scale + camera_position.x,
                cursor_position.y as f32 * ortho_projection.scale + camera_position.y,
            );

            // Check if the cursor is hovering over an agent.
            for (entity, transform) in agent_query.iter_mut() {
                let agent_position = Vec2::new(transform.translation.x, transform.translation.y);

                // If the distance to the agent is within the detection radius,
                // set this agent as the currently hovered agent.
                if distance(cursor_position_scaled, agent_position) < AGENT_DETECTION_RADIUS {
                    hovered_agent.entity = Some(entity);
                    let cursor_position = Vec2::new(
                        cursor_position.x as f32,
                        primary_window.height() - cursor_position.y as f32,
                    );
                    hovered_agent.position = cursor_position;
                    break;
                }
            }
        }
    }
}

// Function to display information of the currently hovered agent.
fn display_info_of_currently_hovered_agent(
    mut commands: Commands,
    hovered_agent: Res<CurrentlyHoveredAgent>,
    asset_server: Res<AssetServer>,
    agent_query: Query<&Agent>,
    mut ui_query: Query<(Entity, &Children), With<AgentInfoDisplayUI>>,
) {
    if let Some(agent_entity) = hovered_agent.entity {
        if let Ok(agent) = agent_query.get(agent_entity) {
            // Define the style for the information display.
            let style = Style {
                size: Size::new(Val::Px(200.0), Val::Px(100.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(hovered_agent.position.x),
                    top: Val::Px(hovered_agent.position.y),
                    ..Default::default()
                },
                ..Default::default()
            };

            // Load the custom font.
            let font = asset_server.load("fonts/FiraSans-Bold.ttf");

            // Define the text bundle for the information to be displayed.
            let text_bundle = TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexStart,
                    justify_content: JustifyContent::FlexStart,
                    ..Default::default()
                },
                text: Text::from_section(
                    format!("{}", agent),
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                ..Default::default()
            };

            // Despawn any existing UI components before creating the new UI.
            for (entity, children) in ui_query.iter_mut() {
                for child in children.iter() {
                    commands.entity(*child).despawn();
                }
                commands.entity(entity).despawn();
            }

            // Create the new UI for displaying agent information.
            commands
                .spawn(NodeBundle {
                    style,
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                    ..Default::default()
                })
                .insert(AgentInfoDisplayUI)
                .with_children(|parent| {
                    parent.spawn(text_bundle);
                });
        }
    } else {
        // If no agent is being hovered over, despawn the existing UI components.
        for (entity, children) in ui_query.iter_mut() {
            for child in children.iter() {
                commands.entity(*child).despawn();
            }
            commands.entity(entity).despawn();
        }
    }
}

pub struct AgentHoverPlugin;

impl Plugin for AgentHoverPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentlyHoveredAgent::default())
            .add_system(detect_currently_hovered_agent.in_set(OnUpdate(AppState::InGame)))
            .add_system(display_info_of_currently_hovered_agent.in_set(OnUpdate(AppState::InGame)));
    }
}
