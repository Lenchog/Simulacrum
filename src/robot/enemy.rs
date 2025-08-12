use crate::{prelude::*, weapons::prelude::*};

#[derive(Component, PartialEq, Default)]
#[require(Robot, Recoil, Hookable)]
pub struct Enemy;

#[derive(Bundle, Default, LdtkEntity)]
pub struct EnemyBundle {
    enemy: Enemy,
    #[sprite_sheet]
    sprite: Sprite,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

pub fn add_enemy() -> impl Bundle {
    (
        Enemy,
        (RigidBody::Dynamic, children![EnemyHurtbox, EnemyHitbox]),
    )
}

#[derive(Component)]
#[require(
    RobotCollider,
    CollisionLayers = CollisionLayers::new(
        PhysicsLayers::Enemy,
        [
            PhysicsLayers::Ground,
            PhysicsLayers::PlayerHitbox,
        ],
    )
)]
struct EnemyHurtbox;

#[derive(Component)]
#[require(
    Hitbox,
    CollisionLayers = CollisionLayers::new(
        PhysicsLayers::EnemyHitbox,
        [PhysicsLayers::Player],
    )
)]
struct EnemyHitbox;
