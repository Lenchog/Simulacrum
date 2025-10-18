use crate::prelude::*;

pub struct PhysicsPlugin;

fn tnua_platforms(mut q_tnua: Query<(&mut TnuaProximitySensor, &TnuaGhostSensor)>) {
    const MIN_PROXIMITY: f32 = 3.0;
    for (mut proximity_sensor, ghost_sensor) in q_tnua.iter_mut() {
        for ghost_platform in ghost_sensor.iter() {
            if MIN_PROXIMITY <= ghost_platform.proximity {
                proximity_sensor.output = Some(ghost_platform.clone());
                break;
            }
        }
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian2dPlugin::new(FixedUpdate),
            PhysicsPlugins::default()
                .with_length_unit(20.0)
                .set(PhysicsInterpolationPlugin::interpolate_all()),
        ))
        .add_systems(FixedUpdate, tnua_platforms.in_set(TnuaUserControlsSystems));
    }
}
