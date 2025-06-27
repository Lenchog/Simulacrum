use bevy::prelude::*;

pub mod general_movement;
pub mod player;
pub mod setup;

#[derive(Component, PartialEq)]
pub struct Player;

#[derive(Component, PartialEq)]
pub struct Floor;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Resource)]
pub struct MovementConfig {
    pub jump: f32,
    pub hold_jump: f32,
    pub acceleration: f32,
}

#[derive(Resource)]
pub struct Direction(pub f32);

#[derive(Resource)]
pub struct Actionable(pub bool);

#[derive(Resource)]
pub struct PhysicsEnabled(pub bool);

#[derive(Resource)]
pub struct DoubleJump(pub bool);
