use bevy::prelude::Vec2;
use rand::Rng;

pub fn distance(a: Vec2, b: Vec2) -> f32 {
    let x = a.x - b.x;
    let y = a.y - b.y;
    (x * x + y * y).sqrt()
}

// Add a random number from -extent to extent to x
pub fn add_random(x: f32, extent: f32) -> f32 {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(-extent..extent);
    x + r
}
