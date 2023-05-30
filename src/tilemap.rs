use crate::{camera::GameCamera, menu::AppState};
use bevy::prelude::*;
use noise::{NoiseFn, OpenSimplex};

pub const MAP_SIZE: i32 = 250;
pub const TILE_SIZE: i32 = 16;

#[derive(Component, PartialEq, Eq, Hash, Clone)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub tile_type: TileType,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum TileType {
    Grass,
    Water,
    Sand,
    Mountain,
}

impl Default for TileType {
    fn default() -> Self {
        TileType::Grass
    }
}

impl TileType {
    fn to_color(&self) -> Color {
        match self {
            TileType::Grass => Color::hex("#40ff7c").unwrap(),
            TileType::Water => Color::hex("#2ab8f5").unwrap(),
            TileType::Sand => Color::hex("#ffed69").unwrap(),
            TileType::Mountain => Color::hex("#6fa5bd").unwrap(),
        }
    }
}

#[derive(Resource)]
pub struct Seed {
    pub seed: u32,
}

impl Default for Seed {
    fn default() -> Self {
        Seed {
            seed: rand::random(),
        }
    }
}

#[derive(Resource)]
pub struct Tiles {
    pub tiles: [[TileType; MAP_SIZE as usize]; MAP_SIZE as usize],
}

fn setup_map(mut commands: Commands, seed: Res<Seed>) {
    let map_size = MAP_SIZE as usize;
    let mut map = [[TileType::default(); MAP_SIZE as usize]; MAP_SIZE as usize];
    let noise = OpenSimplex::new(seed.seed);
    
    for x in 0..map_size {
        for y in 0..map_size {
            let value = noise.get([x as f64 / 20.0, y as f64 / 20.0]) as f32;
            if value < -0.4 {
                map[x][y] = TileType::Water;
            } else if value < -0.3 {
                map[x][y] = TileType::Sand;
            } else if value < 0.5 {
                map[x][y] = TileType::Grass;
            } else {
                map[x][y] = TileType::Mountain;
            }
        }
    }
    let mut tiles = [[TileType::default(); MAP_SIZE as usize]; MAP_SIZE as usize];
    for x in 0..map_size {
        for y in 0..map_size {
            let tile = Tile {
                x: x as i32,
                y: y as i32,
                tile_type: map[x][y],
            };
            tiles[x][y] = tile.tile_type;
        }
    }
    commands.insert_resource(Tiles {
        tiles: tiles.clone(),
    });
    for (x, tile_array) in tiles.iter().enumerate() {
        for (y, tile) in tile_array.iter().enumerate() {
            let mut transform = Transform::from_xyz(0.0, 0.0, 0.0);
            transform.translation.x = x as f32 * TILE_SIZE as f32;
            transform.translation.y = y as f32 * TILE_SIZE as f32;
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: tile.to_color(),
                        custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                        ..Default::default()
                    },
                    transform,
                    visibility: Visibility::Hidden,
                    ..Default::default()
                })
                .insert(Tile {
                    x: x as i32,
                    y: y as i32,
                    tile_type: *tile,
                });
        }
    }
}

pub fn show_tiles_in_frame(
    mut query: Query<(&mut Visibility, &Transform), With<Tile>>,
    camera_query: Query<(&Transform, &OrthographicProjection), With<GameCamera>>,
) {
    let (camera_transform, ortho) = camera_query.single();
    let camera_x = camera_transform.translation.x;
    let camera_y = camera_transform.translation.y;

    let camera_width = 1000.0;
    let camera_height = 600.0;
    let camera_left = camera_x - camera_width / 1.5 * ortho.scale;
    let camera_right = camera_x + camera_width / 1.4 * ortho.scale;
    let camera_top = camera_y + camera_height / 1.4 * ortho.scale;
    let camera_bottom = camera_y - camera_height / 1.5 * ortho.scale;
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

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Seed>()
            .add_system(setup_map.in_schedule(OnEnter(AppState::InGame)))
            .add_system(show_tiles_in_frame.in_set(OnUpdate(AppState::InGame)));
    }
}
