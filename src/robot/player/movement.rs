use crate::{
    general_movement::{Direction, Grounded},
    robot::player::{
        input::{Dash, Jump, Move},
        *,
    },
};
use bevy_enhanced_input::prelude::{Fired, Started};

#[derive(Component)]
pub struct DoubleJump;

#[derive(Component, Default)]
pub struct DashTimer(pub Timer);

#[derive(Resource)]
pub struct MovementConfig {
    pub jump: f32,
    pub hold_jump: f32,
    pub acceleration: f32,
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

pub fn update_dash_timer(time: Res<Time>, q_dash: Single<&mut DashTimer, With<Player>>) {
    let mut dash = q_dash.into_inner();
    dash.0.tick(time.delta());
}

pub fn dash(
    _: Trigger<Started<Dash>>,
    q_player: Single<(&mut LinearVelocity, &DashTimer, &Direction), With<Player>>,
    movement_config: Res<MovementConfig>,
) {
    let (mut velocity, dash, direction) = q_player.into_inner();
    if !(dash.0.finished()) {
        return;
    }
    velocity.x = movement_config.dash * direction.0;
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

pub fn move_horizontal(
    trigger: Trigger<Fired<Move>>,
    movement_config: Res<MovementConfig>,
    q_player: Single<(&mut LinearVelocity, &mut Direction), With<Player>>,
) {
    let direction = trigger.value.extend(0.0).x;
    let (mut velocity, mut current_direction) = q_player.into_inner();
    current_direction.0 = direction;
    if velocity.x.abs() < movement_config.acceleration {
        velocity.x = direction * movement_config.acceleration;
    }
}
