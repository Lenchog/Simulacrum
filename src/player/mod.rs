use bevy::prelude::*;

pub mod health;
pub mod input;
pub mod movement;
pub mod weapons;

#[derive(Resource)]
pub struct MovementConfig {
    pub jump: f32,
    pub hold_jump: f32,
    pub acceleration: f32,
}

#[derive(Component)]
pub struct PlayerCollider;

#[derive(Resource)]
pub struct DoubleJump(pub bool);

#[derive(Resource)]
pub struct Direction(pub f32);
