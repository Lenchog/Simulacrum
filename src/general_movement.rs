use crate::robot::player::PlayerCollider;
use bevy_simple_subsecond_system::hot;
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Component)]
pub struct Direction(pub f32);

#[hot]
pub fn is_grounded(collider: Entity, collisions: &Collisions) -> bool {
    for contact_pair in collisions.collisions_with(collider) {
        let normal = &contact_pair.manifolds[0].normal;
        let angle = normal.y.atan2(normal.x).to_degrees() - 90.0;
        if (-45.0..45.0).contains(&angle) {
            return true;
        };
    }
    false
}

#[hot]
pub fn update_grounded(
    mut commands: Commands,
    player: Query<(Entity, &ColliderOf), With<PlayerCollider>>,
    collisions: Collisions,
) {
    for (collider_entity, collider_of) in player {
        let rigid_body_entity = collider_of.body;
        let is_grounded = is_grounded(collider_entity, &collisions);
        let mut commands_entity = commands.entity(rigid_body_entity);

        if is_grounded {
            commands_entity.insert(Grounded);
            return;
        } else {
            commands_entity.remove::<Grounded>()
        };
    }
}
