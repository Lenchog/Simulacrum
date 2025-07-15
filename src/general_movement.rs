use crate::robot::player::movement::CaiyoteFrames;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_simple_subsecond_system::hot;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Component)]
pub struct Direction(pub f32);

#[hot]
pub fn is_currently_grounded(collider: Entity, collisions: &Collisions) -> bool {
    for contact_pair in collisions.collisions_with(collider) {
        let normal = &contact_pair.manifolds[0].normal;
        let angle = normal.y.atan2(normal.x).to_degrees() - 90.0;
        if (-45.0..45.0).contains(&angle) || (-225.0..-135.0).contains(&angle) {
            return true;
        };
    }
    false
}

#[hot]
pub fn update_grounded(
    mut commands: Commands,
    q_collider: Query<(Entity, &ColliderOf)>,
    mut q_body: Query<(&LinearVelocity, &mut CaiyoteFrames)>,
    collisions: Collisions,
) {
    for (collider_entity, collider_of) in q_collider {
        let rigid_body_entity = collider_of.body;
        let is_currently_grounded = is_currently_grounded(collider_entity, &collisions);
        let Ok((velocity, mut caiyote_time)) = q_body.get_mut(rigid_body_entity) else {
            continue;
        };
        let mut commands_entity = commands.entity(rigid_body_entity);

        if is_currently_grounded && velocity.y <= 0.0 {
            *caiyote_time = CaiyoteFrames::default();
        } else {
            caiyote_time.0 = caiyote_time.0.saturating_sub(1);
            commands_entity.remove::<Grounded>();
            // if you have no caiyote time, don't insert grounded
            if caiyote_time.0 == 0 {
                continue;
            }
        }
        commands_entity.insert(Grounded);
        return;
    }
}
