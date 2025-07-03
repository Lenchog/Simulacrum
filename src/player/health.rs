use avian2d::prelude::Collisions;
use bevy::prelude::*;
use bevy::ui::widget::Text;

use crate::{
    Enemy, EnemyCollider, Player,
    player::weapons::{Damage, Projectile},
};

#[derive(Component)]
pub struct Health(pub u32);

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

pub fn get_hits(
    //trigger: Trigger<OnCollisionStart>,
    q_enemies: Query<(Entity, &Children, &mut Health), With<Enemy>>,
    q_projectiles: Query<(Entity, &Damage), With<Projectile>>,
    q_enemy_colliders: Query<Entity, With<EnemyCollider>>,
    collisions: Collisions,
    mut commands: Commands,
) {
    for mut enemy in q_enemies {
        for &child in enemy.1 {
            let collider_entity = q_enemy_colliders.get(child).unwrap();
            for entity in collisions.entities_colliding_with(collider_entity) {
                for (projectile, projectile_damage) in q_projectiles {
                    if entity == projectile {
                        commands.entity(projectile).despawn();
                        enemy.2.0 = enemy.2.0.saturating_sub(projectile_damage.0);
                        dbg!(enemy.2.0);
                    }
                }
            }
        }
    }
}
