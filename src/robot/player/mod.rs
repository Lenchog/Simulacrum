use bevy_simple_subsecond_system::hot;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_enhanced_input::prelude::Actions;

use crate::robot::{
    Health, PhysicsLayers, Recoil, Robot, RobotCollider,
    player::{
        input::NormalMovement,
        movement::{CaiyoteFrames, DashCooldownFrames, DashFrames},
        weapons::{RotationCenter, WeaponTip},
    },
};

pub mod input;
pub mod movement;
pub mod weapons;

#[derive(Bundle, LdtkEntity, Default)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet]
    sprite: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Resource)]
pub struct EquippedWeapons {
    pub left: Option<Entity>,
    pub right: Option<Entity>,
}

#[derive(Component, PartialEq)]
#[require(
    RobotCollider,
    CollisionLayers = CollisionLayers::new(
        PhysicsLayers::Player,
        [
            PhysicsLayers::Ground,
            PhysicsLayers::EnemyHitbox,
        ],
    ),
)]
pub struct PlayerCollider;

#[derive(Component, PartialEq, Default)]
#[require(
    Robot,
    Recoil,
    Health = Health(500),
    Actions<NormalMovement>,
    DashCooldownFrames,
    CaiyoteFrames,
    DashFrames,
)]
pub struct Player;

#[hot]
pub fn add_player() -> impl Bundle {
    (
        Player,
        (
            RigidBody::Dynamic,
            children![PlayerCollider, (RotationCenter, children!(WeaponTip))],
        ),
    )
}
