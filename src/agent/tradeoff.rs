use bevy::prelude::*;
use getset::{Getters, Setters};

#[derive(Component, Clone, Copy, Debug, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Tradeoff {
    lhs_multiplier: f32,
    rhs_multiplier: f32,
}

impl Default for Tradeoff {
    fn default() -> Self {
        Tradeoff {
            lhs_multiplier: rand::random(),
            rhs_multiplier: rand::random(),
        }
    }
}