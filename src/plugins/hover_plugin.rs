use bevy::{prelude::*, window::PrimaryWindow};

use crate::{agent::Agent, math::distance};

use super::{app_state_plugin::AppState, camera_plugin::GameCameraPosition};

const AGENT_RADIUS: f32 = 16.0;

#[derive(Default, Resource, Clone)]
pub struct HoveredAgent {
    pub entity: Option<Entity>,
    pub position: Vec2,
}

#[derive(Component)]
pub struct AgentInfoUI;

fn detect_hovered_agent(
    mut hovered_agent: ResMut<HoveredAgent>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(Entity, &Transform), With<Agent>>,
    ortho_query: Query<&OrthographicProjection>,
    camera: Res<GameCameraPosition>,
) {
    let Ok(primary) = window_query.get_single() else {
        return;
    };
    if let Some(cursor_position) = primary.cursor_position() {
        hovered_agent.entity = None;
        let ortho = ortho_query.single();
        let camera_position = Vec2::new(
            camera.pos.x - (primary.width() / 2.0) * ortho.scale,
            camera.pos.y - (primary.height() / 2.0) * ortho.scale,
        );
        let cursor_position_scaled = Vec2::new(
            cursor_position.x as f32 * ortho.scale + camera_position.x,
            cursor_position.y as f32 * ortho.scale + camera_position.y,
        );
        // Now we can check if the cursor is hovering over an agent
        for (entity, transform) in query.iter_mut() {
            let agent_position = Vec2::new(transform.translation.x, transform.translation.y);
            if distance(cursor_position_scaled, agent_position) < AGENT_RADIUS {
                hovered_agent.entity = Some(entity);
                let cursor_position = Vec2::new(
                    cursor_position.x as f32,
                    primary.height() - cursor_position.y as f32,
                );
                hovered_agent.position = cursor_position;
                break;
            }
        }
    }
}

fn display_hovered_agent_info(
    mut commands: Commands,
    hovered_agent: Res<HoveredAgent>,
    asset_server: Res<AssetServer>,
    agent_query: Query<&Agent>,
    mut ui_query: Query<(Entity, &Children), With<AgentInfoUI>>,
) {
    if let Some(agent_entity) = hovered_agent.entity {
        if let Ok(agent) = agent_query.get(agent_entity) {
            let style = Style {
                size: Size::new(Val::Px(200.0), Val::Px(100.0)),
                // Place it at the cursor position
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(hovered_agent.position.x),
                    top: Val::Px(hovered_agent.position.y),
                    ..Default::default()
                },
                ..Default::default()
            };
            let font = asset_server.load("fonts/FiraSans-Bold.ttf");

            let text_bundle = TextBundle {
                style: Style {
                    // Center the text
                    align_self: AlignSelf::FlexStart,
                    justify_content: JustifyContent::FlexStart,
                    ..Default::default()
                },
                // Use the custom font and set the font size
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

            // Remove the old UI
            for (entity, children) in ui_query.iter_mut() {
                for child in children.iter() {
                    commands.entity(*child).despawn();
                }
                commands.entity(entity).despawn();
            }

            // Create the new UI
            commands
                .spawn(NodeBundle {
                    style,
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                    ..Default::default()
                })
                .insert(AgentInfoUI)
                .with_children(|parent| {
                    parent.spawn(text_bundle);
                });
        }
    } else {
        // If there's no hovered agent, remove the old UI
        for (entity, children) in ui_query.iter_mut() {
            for child in children.iter() {
                commands.entity(*child).despawn();
            }
            commands.entity(entity).despawn();
        }
    }
}

pub struct HoverPlugin;

impl Plugin for HoverPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HoveredAgent::default())
            .add_system(detect_hovered_agent.in_set(OnUpdate(AppState::InGame)))
            .add_system(display_hovered_agent_info.in_set(OnUpdate(AppState::InGame)));
    }
}
