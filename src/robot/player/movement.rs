use crate::{
    general_movement::{Direction, Grounded},
    robot::player::{
        input::{Dash, Jump, Move},
        *,
    },
};
use bevy_enhanced_input::prelude::{Fired, Started};
use bevy_simple_subsecond_system::hot;

#[derive(Component)]
pub struct DoubleJump;

#[derive(Component, Default)]
pub struct DashTimer(pub Timer);

#[derive(Component, Default)]
pub struct DashCooldown(pub Timer);

#[derive(Resource)]
pub struct MovementConfig {
    pub jump: f32,
    pub hold_jump: f32,
    pub speed: f32,
    pub dash: f32,
}

pub fn jump(
    _: Trigger<Started<Jump>>,
    player: Single<
        (
            Entity,
            &mut LinearVelocity,
            Option<&Grounded>,
            Option<&DoubleJump>,
        ),
        With<Player>,
    >,
    movement_config: Res<MovementConfig>,
    mut commands: Commands,
) {
    let (entity, mut velocity, grounded, double_jump) = player.into_inner();
    // only jump if you're either grounded or have a double jump
    if !(grounded.is_some() || double_jump.is_some()) {
        return;
    }
    if grounded.is_some() {
        commands.entity(entity).insert(DoubleJump);
    } else {
        commands.entity(entity).remove::<DoubleJump>();
    };
    velocity.y = movement_config.jump;
}

#[hot]
pub fn update_dash_timer(
    time: Res<Time>,
    q_player: Single<(Entity, &mut DashCooldown, &mut DashTimer, Option<&Dashing>), With<Player>>,
    mut commands: Commands,
) {
    let (entity, mut cooldown, mut timer, dashing) = q_player.into_inner();
    cooldown.0.tick(time.delta());
    if dashing.is_some() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(entity).remove::<Dashing>();
            commands.entity(entity).insert(GravityScale(1.0));
            commands.entity(entity).insert(LinearVelocity::ZERO);
            timer.0.reset();
        }
    }
}

#[derive(Component)]
#[require(GravityScale = GravityScale(0.0))]
pub struct Dashing;

pub fn dash(
    _: Trigger<Started<Dash>>,
    q_player: Single<(Entity, &mut LinearVelocity, &Direction, &mut DashCooldown), With<Player>>,
    movement_config: Res<MovementConfig>,
    mut commands: Commands,
) {
    let (entity, mut velocity, direction, mut cooldown) = q_player.into_inner();
    if !(cooldown.0.finished()) {
        return;
    }
    cooldown.0.reset();
    commands.entity(entity).insert(Dashing);
    commands.entity(entity).insert(GravityScale(0.0));
    velocity.x = movement_config.dash * direction.0;
    velocity.y = 0.0;
}

pub fn hold_jump(
    _: Trigger<Fired<Jump>>,
    mut velocity: Query<&mut LinearVelocity, With<Player>>,
    movement_config: Res<MovementConfig>,
    time: Res<Time>,
) {
    let velocity = &mut velocity.single_mut().expect("Player not found!").y;
    if *velocity <= 0.0 {
        return;
    };
    *velocity += movement_config.hold_jump * time.delta_secs() * 62.5;
}

#[hot]
pub fn move_horizontal(
    trigger: Trigger<Fired<Move>>,
    movement_config: Res<MovementConfig>,
    q_player: Single<(&mut LinearVelocity, &mut Direction), With<Player>>,
) {
    let direction = trigger.value.x;
    let (mut velocity, mut current_direction) = q_player.into_inner();
    current_direction.0 = direction.signum();
    if velocity.x.abs() < movement_config.speed {
        velocity.x = direction * movement_config.speed;
    }
}
