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

#[hot]
pub fn enemy_move(
    q_enemies: Query<(&mut TnuaController, &GlobalTransform), With<Enemy>>,
    q_player: Single<&GlobalTransform, With<Player>>,
    r_movement_config: Res<MovementConfig>,
) {
    for (mut controller, transform) in q_enemies {
        let distance = q_player.translation() - transform.translation();
        let direction = distance.x.signum();
        controller.basis(TnuaBuiltinWalk {
            desired_velocity: Vec3::X * direction * r_movement_config.speed,
            acceleration: r_movement_config.accel,
            air_acceleration: r_movement_config.air_accel,
            float_height: 100.0,
            ..Default::default()
        });
    }
}
