use crate::*;
use bevy::math::VectorSpace;

pub fn gravity(
    time: Res<Time<Fixed>>,
    movement_config: Res<MovementConfig>,
    mut sprite_state: Query<(&mut MovementState, Option<&mut Player>)>,
    mut double_jump: ResMut<DoubleJump>,
) {
    for (mut state, player) in &mut sprite_state {
        if touching_floor(state.position) {
            state.velocity.y -= movement_config.gravity * time.delta_secs();
        } else if state.velocity.y < 0.0 {
            state.velocity.y = 0.0;
            if player.is_some() {
                double_jump.0 = true;
            }
        }
    }
}

pub fn touching_floor(sprite_position: Vec2) -> bool {
    sprite_position.y >= -050.0
}

pub fn update_movement(mut sprite_position: Query<&mut MovementState>) {
    for mut state in &mut sprite_position {
        state.position.x += state.velocity.x;
        state.position.y += state.velocity.y;
    }
}

pub fn smooth_movement(time: Res<Time<Fixed>>, mut query: Query<(&mut Transform, &MovementState)>) {
    for (mut transform, state) in &mut query {
        let a = time.overstep_fraction();
        let future_position = Vec2 {
            x: state.position.x + state.velocity.x * time.delta_secs(),
            y: state.position.y + state.velocity.y * time.delta_secs(),
        };
        transform.translation.x = VectorSpace::lerp(state.position.x, future_position.x, a);
        transform.translation.y = VectorSpace::lerp(state.position.y, future_position.y, a);
    }
}

pub fn check_actionable(actionable: Res<Actionable>, physics: Res<Physics>) -> bool {
    actionable.0 && physics.0
}

pub fn check_physics(physics: Res<Physics>) -> bool {
    physics.0
}
