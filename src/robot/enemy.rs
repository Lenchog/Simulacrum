use crate::prelude::*;

#[derive(Component, PartialEq, Default)]
#[require(Robot, Recoil, Hookable)]
pub struct Enemy;

#[derive(Bundle, Default, LdtkEntity)]
pub struct EnemyBundle {
    enemy: Enemy,
    health: Health,
    #[sprite_sheet]
    sprite: Sprite,
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
pub struct EnemyHitbox;
