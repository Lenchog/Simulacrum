use crate::Unhook;
use crate::prelude::*;
use bevy_enhanced_input::prelude::*;

#[derive(Component)]
pub struct DoubleJump;

#[derive(Component, Default)]
pub struct RespawnPoint(pub GridCoords);

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
    _: Trigger<Fired<Jump>>,
    q_controller: Single<&mut TnuaController>,
    movement_config: Res<MovementConfig>,
    mut ev_unhook: EventWriter<Unhook>,
) {
    let mut controller = q_controller.into_inner();
    ev_unhook.write(Unhook);
    controller.action(TnuaBuiltinJump {
        height: movement_config.jump,
        ..Default::default()
    });
}

pub fn dash(
    _: Trigger<Started<Dash>>,
    q_controller: Single<(&mut TnuaController, &mut Direction)>,
    movement_config: Res<MovementConfig>,
) {
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
