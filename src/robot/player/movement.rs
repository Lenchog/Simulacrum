use crate::{prelude::*, weapons::prelude::Unhook};
use bevy_enhanced_input::prelude::*;
use bevy_tnua::control_helpers::TnuaSimpleAirActionsCounter;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MovementConfig {
            jump: 500.0,
            dash_speed: 2400.0,
            dash: 400.0,
            dash_accel: 30000.0,
            dash_decel: 6000.0,
            speed: 1000.0,
            accel: 8000.0,
            air_accel: 6000.0,
        })
        .insert_resource(Gravity(Vec2::NEG_Y * 12000.0))
        .add_systems(FixedUpdate, move_horizontal);
    }
}

#[derive(Component)]
pub struct DoubleJump;

#[derive(Component)]
pub struct Direction(pub f32);
impl Default for Direction {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Resource)]
pub struct MovementConfig {
    pub jump: f32,
    pub speed: f32,
    pub accel: f32,
    pub air_accel: f32,
    pub dash: f32,
    pub dash_accel: f32,
    pub dash_decel: f32,
    pub dash_speed: f32,
}

pub fn jump(
    _: Trigger<Started<Jump>>,
    q_controller: Single<(&mut TnuaController, &mut TnuaSimpleAirActionsCounter)>,
    movement_config: Res<MovementConfig>,
    r_unlocks: Res<Unlocks>,
    mut ev_unhook: EventWriter<Unhook>,
) {
    let (mut controller, mut air_actions) = q_controller.into_inner();
    air_actions.update(controller.as_ref());
    let max_air_actions = if r_unlocks.double_jump { 1 } else { 0 };
    ev_unhook.write(Unhook);
    controller.action(TnuaBuiltinJump {
        height: movement_config.jump,
        allow_in_air: air_actions.air_count_for(TnuaBuiltinJump::NAME) <= max_air_actions,
        ..Default::default()
    });
}

pub fn dash(
    _: Trigger<Started<Dash>>,
    q_controller: Single<(&mut TnuaController, &mut Direction)>,
    r_unlocks: Res<Unlocks>,
    movement_config: Res<MovementConfig>,
) {
    if !r_unlocks.dash {
        return;
    }
    let (mut controller, direction) = q_controller.into_inner();
    controller.action(TnuaBuiltinDash {
        displacement: Vec3::X * movement_config.dash * direction.0,
        speed: movement_config.dash_speed,
        acceleration: movement_config.dash_accel,
        brake_acceleration: movement_config.dash_decel,
        brake_to_speed: movement_config.speed,
        allow_in_air: true,
        ..default()
    })
}

#[hot]
pub fn move_horizontal(
    q_actions: Query<&Action<MoveAction>>,
    movement_config: Res<MovementConfig>,
    q_player: Single<(&mut TnuaController, &mut Direction), With<Player>>,
) {
    let (mut controller, mut current_direction) = q_player.into_inner();
    for action in q_actions {
        let direction = action.x;
        controller.basis(TnuaBuiltinWalk {
            desired_velocity: Vec3::X * direction * movement_config.speed,
            acceleration: movement_config.accel,
            air_acceleration: movement_config.air_accel,
            float_height: 100.0,
            ..Default::default()
        });
        if direction == 0.0 {
            return;
        }
        current_direction.0 = direction;
    }
}
