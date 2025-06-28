use crate::*;
use avian2d::prelude::*;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Resource)]
pub struct Actionable(pub bool);

#[derive(Resource)]
pub struct PhysicsEnabled(pub bool);

pub fn is_grounded(floors: Query<Entity, With<Floor>>, collisions: Collisions) -> bool {
    for floor in floors {
        for contact_pair in collisions.collisions_with(floor) {
            let normal = &contact_pair.manifolds[0].normal;
            let angle = normal.y.atan2(normal.x).to_degrees() - 90.0;
            return (-45.0..45.0).contains(&angle);
        }
    }
    false
}

pub fn update_grounded(
    mut commands: Commands,
    query: Query<Entity, With<Player>>,
    floors: Query<Entity, With<Floor>>,
    collisions: Collisions,
) {
    let is_grounded: bool = is_grounded(floors, collisions);
    let entity = query.single().expect("could not find player!");

    let mut commands_entity = commands.entity(entity);
    if is_grounded {
        commands_entity.insert(Grounded)
    } else {
        commands_entity.remove::<Grounded>()
    };
}

pub fn check_actionable(actionable: Res<Actionable>, physics: Res<PhysicsEnabled>) -> bool {
    actionable.0 && physics.0
}

pub fn check_physics(physics: Res<PhysicsEnabled>) -> bool {
    physics.0
}
