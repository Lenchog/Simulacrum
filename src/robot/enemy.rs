use crate::{prelude::*, weapons::prelude::*};

#[derive(Component, PartialEq, Default)]
#[require(Robot, Recoil, Hookable, AggroState)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (enemy_move, update_aggro_state));
    }
}

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
    CollisionLayers::new(
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
    CollisionLayers::new(
        PhysicsLayers::EnemyHitbox,
        [PhysicsLayers::Player],
    )
)]
struct EnemyHitbox;

#[derive(Component, PartialEq, Default, Debug)]
pub enum AggroState {
    #[default]
    Idle,
    Wary,
    Aggro,
    Chase,
}

fn update_aggro_state(
    q_enemies: Query<(&mut AggroState, &GlobalTransform)>,
    q_player: Single<&GlobalTransform, With<Player>>,
) {
    for (mut aggro_state, transform) in q_enemies {
        let distance = q_player.translation().distance(transform.translation());
        *aggro_state = match *aggro_state {
            AggroState::Idle | AggroState::Wary => match distance {
                ..1024.0 => AggroState::Aggro,
                ..1536.0 => AggroState::Wary,
                _ => AggroState::Idle,
            },
            _ => match distance {
                ..768.0 => AggroState::Chase,
                ..1024.0 => AggroState::Aggro,
                ..2048.0 => AggroState::Wary,
                _ => AggroState::Idle,
            },
        }
    }
}

fn enemy_move(
    q_enemies: Query<(&mut TnuaController, &GlobalTransform, &AggroState), With<Enemy>>,
    q_player: Single<&GlobalTransform, With<Player>>,
    r_movement_config: Res<MovementConfig>,
) {
    for (mut controller, transform, aggro_state) in q_enemies {
        let distance = q_player.translation() - transform.translation();
        let direction = distance.x.signum();
        let aggro_mult = match aggro_state {
            AggroState::Idle => 0.0,
            AggroState::Wary => 0.1,
            AggroState::Aggro => 0.5,
            AggroState::Chase => 0.7,
        };
        controller.basis(TnuaBuiltinWalk {
            desired_velocity: Vec3::X * direction * r_movement_config.speed * aggro_mult,
            acceleration: r_movement_config.accel,
            air_acceleration: r_movement_config.air_accel,
            float_height: 100.0,
            ..Default::default()
        });
    }
}
