use crate::robot::{Health, player::Player};
use bevy::prelude::*;
use bevy::ui::widget::Text;
use bevy_simple_subsecond_system::hot;

#[derive(Component)]
pub struct HealthBar;

#[hot]
pub fn update_player_health_bar(
    health_bar: Single<&mut Text, With<HealthBar>>,
    player_health: Single<&Health, With<Player>>,
) {
    *health_bar.into_inner() = Text::new(player_health.into_inner().0.to_string());
}
