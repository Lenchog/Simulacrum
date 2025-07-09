use avian2d::prelude::{LinearVelocity, OnCollisionStart};
use bevy::prelude::*;
use bevy::ui::widget::Text;

use crate::{
    Despawnable, Recoil,
    robot::{Hitbox, Robot, player::Player},
};

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Component, Clone)]
pub struct Damage(pub u32);

#[derive(Component)]
pub struct HealthBar;

pub fn update_player_health_bar(
    mut health_bar: Query<&mut Text, With<HealthBar>>,
    player_health: Query<&Health, With<Player>>,
) {
    *health_bar.single_mut().expect("Could not find health bar!") = Text::new(
        player_health
            .single()
            .expect("couldn't find player health")
            .0
            .to_string(),
    );
}

#[derive(Event)]
pub struct HitEvent(Entity, Entity, Damage, f32);

pub fn get_hits(
    trigger: Trigger<OnCollisionStart>,
    mut q_hitboxes: Query<(Entity, &Damage), With<Hitbox>>,
    q_transform: Query<&GlobalTransform>,
    mut ev_hit: EventWriter<HitEvent>,
) {
    let hitbox = trigger.target();
    let hurtbox = trigger.body.unwrap();
    let velocity_direction_mult =
        if get_entity_x(q_transform, hurtbox) - get_entity_x(q_transform, hitbox) < 0.0 {
            -1.0
        } else {
            1.0
        };
    if let Ok((_, damage)) = q_hitboxes.get_mut(hitbox) {
        ev_hit.write(HitEvent(
            hitbox,
            hurtbox,
            damage.clone(),
            velocity_direction_mult,
        ));
    }
}

fn get_entity_x(q_transform: Query<&GlobalTransform>, entity: Entity) -> f32 {
    q_transform
        .get(entity)
        .expect("entity does not have transform")
        .translation()
        .x
}

pub fn got_hit(
    mut ev_hit: EventReader<HitEvent>,
    mut q_robots: Query<(&mut Health, &mut LinearVelocity), With<Robot>>,
) {
    for event in ev_hit.read() {
        let hurtbox = event.1;
        let Ok((mut health, mut velocity)) = q_robots.get_mut(hurtbox) else {
            continue;
        };
        health.0 -= event.2.0;
        // knockback
        **velocity = Vec2 {
            x: 1000.0 * event.3,
            y: 2000.0,
        };
    }
}

pub fn hit_something(
    mut ev_hit: EventReader<HitEvent>,
    q_despawnable: Query<&Despawnable>,
    q_health: Query<&Health>,
    mut commands: Commands,
    mut q_recoil: Query<&mut LinearVelocity, With<Recoil>>,
    q_children: Query<&ChildOf>,
) {
    for event in ev_hit.read() {
        if let Ok(parent) = q_children.get(event.0)
            && let Ok(mut velocity) = q_recoil.get_mut(parent.0)
            && q_health.contains(event.1)
        {
            // recoil, opposite of knockback
            **velocity = Vec2 {
                x: 500.0 * -event.3,
                y: 0.0,
            };
        }
        if q_despawnable.contains(event.0) {
            commands.entity(event.0).try_despawn();
        }
    }
}
