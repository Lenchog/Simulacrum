use bevy::prelude::*;

pub mod general_movement;
pub mod player;
pub mod setup;

#[derive(Component, PartialEq)]
pub struct Player;

#[derive(Component, PartialEq)]
pub struct MovementState {
    position: Vec2,
    velocity: Vec2,
}

#[derive(Resource)]
pub struct MovementConfig {
    pub gravity: f32,
    pub jump: f32,
    pub hold_jump: f32,
    pub acceleration: f32,
    pub decelleration: f32,
    pub friction: f32,
}

#[derive(Resource)]
pub struct Direction(pub f32);

#[derive(Resource)]
pub struct Actionable(pub bool);

#[derive(Resource)]
pub struct Physics(pub bool);

#[derive(Resource)]
pub struct DoubleJump(pub bool);
