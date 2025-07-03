use bevy::prelude::*;

pub mod general_movement;
pub mod player;
pub mod setup;

#[derive(Component, PartialEq)]
pub struct Player;

#[derive(Component, PartialEq)]
pub struct Enemy;

#[derive(Component, PartialEq)]
pub struct EnemyCollider;

#[derive(Component, PartialEq)]
pub struct Floor;

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Component)]
pub struct HealthBar;

#[derive(Resource)]
pub struct MouseCoordinates(pub Vec2);
