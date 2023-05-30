use bevy::prelude::Vec2;

pub fn distance(a: Vec2, b: Vec2) -> f32 {
    let x = a.x - b.x;
    let y = a.y - b.y;
    (x * x + y * y).sqrt()
}
