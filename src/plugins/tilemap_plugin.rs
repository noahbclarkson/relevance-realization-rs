use bevy::prelude::*;
use noise::{NoiseFn, OpenSimplex};
use rand::Rng;

use crate::{
    positioning::TilePosition,
    tilemap::{Seed, Tile, TileType, Tiles, MAP_SIZE, TILE_SIZE},
};

use super::{
    agent_creation_plugin::AgentCreationPlugin, app_state_plugin::AppState,
    performance_plugin::PerformancePlugin,
};

fn generate_tilemap(seed: u32) -> [[TileType; MAP_SIZE as usize]; MAP_SIZE as usize] {
    let mut map = [[TileType::default(); MAP_SIZE as usize]; MAP_SIZE as usize];
    let noise = OpenSimplex::new(seed);
    let mut rand = rand::thread_rng();

    for x in 0..MAP_SIZE as usize {
        for y in 0..MAP_SIZE as usize {
            let mut value = noise.get([x as f64 / 25.0, y as f64 / 25.0]) as f32 * 2.0;
            value += rand.gen_range(-0.02..0.02);
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

    map
}

fn setup_map(mut commands: Commands, seed: Res<Seed>, mut tiles: ResMut<Tiles>) {
    tiles.tiles = generate_tilemap(seed.tile_seed);
    let mut rand = rand::thread_rng();

    for (x, tile_array) in tiles.tiles.iter().enumerate() {
        for (y, tile) in tile_array.iter().enumerate() {
            let tile_size = TILE_SIZE as f32;
            let transform = Transform::from_xyz(x as f32 * tile_size, y as f32 * tile_size, 0.0);
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: tile.to_color(&mut rand).unwrap(),
                        custom_size: Some(Vec2::new(tile_size, tile_size)),
                        ..Default::default()
                    },
                    transform,
                    visibility: Visibility::Hidden,
                    ..Default::default()
                })
                .insert(Tile {
                    position: TilePosition::new(x as i32, y as i32),
                    tile_type: *tile,
                });
        }
    }
}

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Seed>()
            .init_resource::<Tiles>()
            .add_system(setup_map.in_schedule(OnEnter(AppState::InGame)))
            .add_plugin(PerformancePlugin)
            .add_plugin(AgentCreationPlugin);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_tilemap() {
        let seed = 1; // For reproducible results, choose a fixed seed.
        let map = generate_tilemap(seed);
        assert_eq!(map.len(), MAP_SIZE as usize);
        for row in map.iter() {
            assert_eq!(row.len(), MAP_SIZE as usize);
        }
    }
}
