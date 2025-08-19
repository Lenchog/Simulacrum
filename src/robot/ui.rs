use crate::{prelude::*, setup::AppState};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (update_ui, button_system));
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
fn update_ui(
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
        padding: UiRect::all(Val::Px(30.0)),
        ..default()
    };

    let button = (
        Button,
        BorderColor::from(Color::BLACK),
        Node {
            width: Val::Px(600.),
            height: Val::Px(100.),
            border: UiRect::all(Val::Px(1000.)),
            ..default()
        },
        Text::new("Start Game"),
        TextColor(Color::BLACK),
    );

    let button_entity = commands.spawn(button).id();
    commands
        .spawn((container, StateScoped(AppState::MainMenu)))
        .add_children(&[button_entity]);
}

const NORMAL_BUTTON: Color = Color::srgb(0.4, 0.4, 0.4);
const HOVER_BUTTON: Color = Color::srgb(0.6, 0.6, 0.6);

#[hot]
fn button_system(
    mut q_interactions: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color) in q_interactions.iter_mut() {
        match *interaction {
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVER_BUTTON.into();
            }
            Interaction::Pressed => {
                *color = HOVER_BUTTON.into();
                next_state.set(AppState::InGame);
            }
        }
    }
}
