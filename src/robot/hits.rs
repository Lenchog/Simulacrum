use crate::{prelude::*, weapons::prelude::*};
use bevy::prelude::ops::sqrt;
use bevy_ecs_ldtk::utils::grid_coords_to_translation;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<HitMessage>()
            .add_systems(FixedUpdate, (got_hit, hit_something, tick_iframes));
    }
}

#[derive(Message)]
pub struct DeathMessage(pub Entity);

#[derive(Component)]
pub struct IFrames(pub u8);

#[derive(Message)]
pub struct HitMessage(Entity, Entity, Damage, f32);

#[derive(Message)]
pub struct CollectableMessage(pub Entity, pub CollectableType);

pub fn get_hits(
    add: On<CollisionStart>,
    mut q_hitboxes: Query<&Damage, With<Hitbox>>,
    q_collectable: Query<(Entity, &CollectableType)>,
    q_transform: Query<&GlobalTransform>,
    mut ev_hit: MessageWriter<HitMessage>,
    mut ev_collectable: MessageWriter<CollectableMessage>,
) {
    let hitbox = add.event().event_target();
    let Some(hurtbox) = add.body1 else {
        return;
    };
    let hurtbox_x = q_transform
        .get(hurtbox)
        .expect("entity does not have transform")
        .translation()
        .x;
    let hitbox_x = q_transform
        .get(hitbox)
        .expect("entity does not have transform")
        .translation()
        .x;
    let velocity_direction_mult = if hurtbox_x - hitbox_x < 0.0 {
        -1.0
    } else {
        1.0
    };
    if let Ok(damage) = q_hitboxes.get_mut(hitbox) {
        ev_hit.write(HitMessage(
            hitbox,
            hurtbox,
            damage.clone(),
            velocity_direction_mult,
        ));
    } else if let Ok((entity, collectable_type)) = q_collectable.get(hitbox) {
        ev_collectable.write(CollectableMessage(entity, collectable_type.clone()));
    }
}

pub fn got_hit(
    mut ev_hit: MessageReader<HitMessage>,
    mut ev_death: MessageWriter<DeathMessage>,
    mut shake: Single<&mut Shake>,
    mut q_robots: Query<
        (
            &mut Transform,
            &mut Health,
            &mut LinearVelocity,
            Option<&Player>,
        ),
        With<Robot>,
    >,
    q_iframes: Query<&mut IFrames>,
    q_respawn_point: Single<&RespawnPoint>,
    q_hooked: Query<&Hooked>,
    q_spikes: Query<&Spike>,
    q_player_hitbox: Query<&PlayerHitbox>,
    q_projectile_type: Query<&ProjectileType>,
    q_energy: Single<&mut Energy>,
    r_unlocks: Res<Unlocks>,
    mut commands: Commands,
) {
    let mut energy = q_energy.into_inner();
    for event in ev_hit.read() {
        let (hitbox, hurtbox, mut damage, knockback) = (event.0, event.1, event.2.0, &event.3);
        if q_iframes.contains(hurtbox) {
            continue;
        }
        if let Ok(projectile) = q_projectile_type.get(hitbox)
            && *projectile == ProjectileType::Hook
            && q_hooked.is_empty()
        {
            commands.entity(hurtbox).insert(Hooked);
        }
        let Ok((mut transform, mut health, mut velocity, player)) = q_robots.get_mut(hurtbox)
        else {
            continue;
        };
        commands.entity(hurtbox).insert(IFrames(30));
        if q_player_hitbox.contains(hitbox) {
            energy.0 += 3;
            damage = damage + damage * energy.0 / 100;
            if energy.0 > r_unlocks.max_energy {
                energy.0 = r_unlocks.max_energy;
            }
        }

        // More screenshake if the player is hit
        let divisor = if player.is_some() { 6.0 } else { 15.0 };
        shake.add_trauma(sqrt(damage as f32) / divisor);
        health.0 = health.0.saturating_sub(damage);
        if health.0 == 0 {
            ev_death.write(DeathMessage(hurtbox));
        }
        if q_spikes.contains(hitbox) {
            *transform = Transform::from_translation(
                grid_coords_to_translation(q_respawn_point.0, IVec2::splat(128)).extend(0.0),
            );
            // no knockback if respawning
            continue;
        }
        // knockback
        **velocity = Vec2 {
            x: 1000.0 * knockback,
            y: 2000.0,
        };
    }
}

fn tick_iframes(q_iframes: Query<(&mut IFrames, Entity)>, mut commands: Commands) {
    for (mut iframes, entity) in q_iframes {
        iframes.0 -= 1;
        if iframes.0 == 0 {
            commands.entity(entity).remove::<IFrames>();
        }
    }
}

fn get_ancestor_recoil(
    entity: Entity,
    q_velocity: &Query<&mut LinearVelocity, With<Recoil>>,
    q_parents: Query<&ChildOf>,
) -> Option<Entity> {
    let Ok(parent) = q_parents.get(entity) else {
        return None;
    };
    if q_velocity.contains(parent.0) {
        Some(parent.0)
    } else {
        get_ancestor_recoil(parent.0, q_velocity, q_parents)
    }
}

pub fn hit_something(
    mut ev_hit: MessageReader<HitMessage>,
    q_health: Query<&Health>,
    mut q_velocity: Query<&mut LinearVelocity, With<Recoil>>,
    q_projectile_type: Query<&ProjectileType>,
    q_parents: Query<&ChildOf>,
    q_transform: Query<&Transform>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for event in ev_hit.read() {
        let (hitbox, hurtbox, knockback_mult) = (event.0, event.1, event.3);
        if let Some(parent) = get_ancestor_recoil(hitbox, &q_velocity, q_parents)
            && let Ok(mut velocity) = q_velocity.get_mut(parent)
            && q_health.contains(hurtbox)
        {
            // recoil
            **velocity = Vec2 {
                x: 1000.0 * -knockback_mult,
                y: 0.0,
            };
        }
        if let Ok(projectile_type) = q_projectile_type.get(hitbox) {
            match *projectile_type {
                ProjectileType::Rocket => {
                    commands.spawn((explosion(&asset_server), *q_transform.get(hitbox).unwrap()));
                }
                ProjectileType::Hook => {
                    commands.entity(hitbox).insert(Retracting);
                    return;
                }
                _ => {}
            }
            commands.entity(hitbox).try_despawn();
        }
    }
}
