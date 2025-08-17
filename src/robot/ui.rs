use crate::{prelude::*, setup::AppState};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_ui);
    }
}
#[derive(Component)]
#[require(Text)]
pub struct HealthBar;

#[derive(Component)]
#[require(
    Text,
    Node {
        top: Val::Px(16.0),
        ..default()
    }
)]
pub struct EnergyBar;

#[hot]
pub fn update_ui(
    q_health_bar: Single<&mut Text, (With<HealthBar>, Without<EnergyBar>)>,
    q_energy_bar: Single<&mut Text, With<EnergyBar>>,
    q_player: Single<(&Health, &Energy), With<Player>>,
) {
    let (health, energy) = q_player.into_inner();
    *q_health_bar.into_inner() = Text::new(health.0.to_string());
    *q_energy_bar.into_inner() = Text::new(energy.0.to_string());
}

#[hot]
pub fn main_menu(mut commands: Commands) {
    let container = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        ..default()
    };

    let square = (
        BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
        Node {
            width: Val::Px(200.),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
    );

    commands.spawn((
        (container, children![square]),
        StateScoped(AppState::MainMenu),
    ));
}
