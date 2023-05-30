use bevy::prelude::*;
use noise::{NoiseFn, OpenSimplex};

use crate::{tilemap::{Seed, Tile, TileType, Tiles, MAP_SIZE, TILE_SIZE}, math::add_random};

use super::{app_state_plugin::AppState, performance_plugin::PerformancePlugin};

fn setup_map(mut commands: Commands, seed: Res<Seed>) {
    let map_size = MAP_SIZE as usize;
    let mut map = [[TileType::default(); MAP_SIZE as usize]; MAP_SIZE as usize];
    let noise = OpenSimplex::new(seed.tile_seed);

    for x in 0..map_size {
        for y in 0..map_size {
            let value = noise.get([x as f64 / 25.0, y as f64 / 25.0]) as f32 * 2.0;
            let value = add_random(value, 0.02);
            if value < -0.8 {
                map[x][y] = TileType::DeepWater;
            } else if value < -0.55 {
                map[x][y] = TileType::Water;
            } else if value < -0.35 {
                map[x][y] = TileType::Sand;
            } else if value < 0.75 {
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

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Seed>()
            .add_system(setup_map.in_schedule(OnEnter(AppState::InGame)))
            .add_plugin(PerformancePlugin);
    }
}
