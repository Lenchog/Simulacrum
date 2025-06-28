use bevy::prelude::*;

pub mod input;
pub mod movement;

#[derive(Resource)]
pub struct MovementConfig {
    pub jump: f32,
    pub hold_jump: f32,
    pub acceleration: f32,
}

#[derive(Resource)]
pub struct DoubleJump(pub bool);

#[derive(Resource)]
pub struct Direction(pub f32);
