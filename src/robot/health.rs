use avian2d::prelude::OnCollisionStart;
use bevy::prelude::*;
use bevy::ui::widget::Text;

use crate::robot::{
    enemy::*,
    player::{Player, weapons::Projectile},
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
    trigger: Trigger<OnCollisionStart>,
    q_projectiles: Query<&Health, (With<Projectile>, Without<Enemy>)>,
    mut q_enemies: Query<&mut Health, With<Enemy>>,
    mut commands: Commands,
) {
    if let Ok(mut health) = q_enemies.get_mut(trigger.body.unwrap())
        && let Ok(damage) = q_projectiles.get(trigger.target())
    {
        health.0 = health.0.saturating_sub(damage.0);
        dbg!(health.0);
    }
    commands.entity(trigger.target()).try_despawn();
}
