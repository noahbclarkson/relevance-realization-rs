use bevy::prelude::{IVec2, Vec2};

pub fn distance(a: IVec2, b: IVec2) -> f32 {
    let x = (a.x - b.x) as f32;
    let y = (a.y - b.y) as f32;
    (x * x + y * y).sqrt()
}

pub fn distance_f32(a: Vec2, b: Vec2) -> f32 {
    let x = a.x - b.x;
    let y = a.y - b.y;
    (x * x + y * y).sqrt()
}