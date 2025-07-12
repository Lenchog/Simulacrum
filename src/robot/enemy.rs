use crate::robot::*;
use bevy_ecs_ldtk::LdtkEntity;
#[derive(Component, PartialEq, Default)]
#[require(Robot)]
pub struct Enemy;

#[derive(Bundle, Default, LdtkEntity)]
pub struct EnemyBundle {
    enemy: Enemy,
    #[sprite_sheet]
    sprite: Sprite,
}

pub fn add_enemy() -> impl Bundle {
    (Enemy, (RigidBody::Dynamic, children![EnemyHitbox]))
}

#[derive(Component)]
#[require(
    RobotCollider,
    CollisionLayers = CollisionLayers::new(
        PhysicsLayers::Enemy,
        [
            PhysicsLayers::Ground,
            PhysicsLayers::Player,
            PhysicsLayers::PlayerProjectile,
        ],
    )
)]
struct EnemyHitbox;
