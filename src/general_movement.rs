use crate::prelude::*;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Component)]
pub struct Direction(pub f32);
impl Default for Direction {
    fn default() -> Self {
        Self(1.0)
    }
}

struct GroundedInfo {
    grounded: bool,
    respawn_ground: bool,
}
impl GroundedInfo {
    fn not_grounded() -> Self {
        Self {
            grounded: false,
            respawn_ground: false,
        }
    }
}

fn grounded_state(
    collider: Entity,
    collisions: &Collisions,
    q_walls: &Query<Option<&Platform>, With<Wall>>,
) -> GroundedInfo {
    for contact_pair in collisions.collisions_with(collider) {
        let wall_entity = if q_walls.contains(contact_pair.collider1) {
            contact_pair.collider1
        } else if q_walls.contains(contact_pair.collider2) {
            contact_pair.collider2
        } else {
            continue;
        };
        let platform = q_walls.get(wall_entity).unwrap();
        let normal = &contact_pair.manifolds[0].normal;
        let angle = normal.y.atan2(normal.x).to_degrees() - 90.0;
        if (-45.0..45.0).contains(&angle) || (-225.0..-135.0).contains(&angle) {
            if platform.is_some() {
                return GroundedInfo {
                    grounded: true,
                    respawn_ground: false,
                };
            } else {
                return GroundedInfo {
                    grounded: true,
                    respawn_ground: true,
                };
            }
        }
    }
    GroundedInfo::not_grounded()
}

#[hot]
pub fn update_grounded(
    mut commands: Commands,
    q_robot: Query<(Entity, &ColliderOf), With<RobotCollider>>,
    mut q_body: Query<(&LinearVelocity, &mut CaiyoteFrames)>,
    q_walls: Query<Option<&Platform>, With<Wall>>,
    q_respawn_point: Single<(&Transform, &mut RespawnPoint), With<Player>>,
    collisions: Collisions,
) {
    let (transform, mut respawn) = q_respawn_point.into_inner();
    for (collider_entity, collider_of) in q_robot {
        let rigid_body_entity = collider_of.body;
        let grounded_state = grounded_state(collider_entity, &collisions, &q_walls);
        let Ok((velocity, mut caiyote_time)) = q_body.get_mut(rigid_body_entity) else {
            continue;
        };
        let mut commands_entity = commands.entity(rigid_body_entity);

        if grounded_state.grounded && velocity.y <= 0.0 {
            *caiyote_time = CaiyoteFrames::default();
        } else {
            caiyote_time.0 = caiyote_time.0.saturating_sub(1);
            commands_entity.remove::<Grounded>();
            // if you have no caiyote time, don't insert grounded
            if caiyote_time.0 == 0 {
                continue;
            }
        }
        if grounded_state.respawn_ground {
            respawn.0 = *transform;
        };
        commands_entity.insert(Grounded);
        return;
    }
}
