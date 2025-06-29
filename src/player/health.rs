use bevy::prelude::*;
use bevy::ui::widget::Text;

use crate::{Health, HealthBar, Player};

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
