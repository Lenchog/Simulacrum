use crate::{prelude::*, setup::AppState};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (update_ui, button_system, handle_click))
            .add_event::<ClickEvent>();
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
        justify_content: JustifyContent::Start,
        padding: UiRect::all(Val::Px(30.0)),
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(15.0),
        ..default()
    };

    let start = button(ButtonType::StartGame);
    let exit = button(ButtonType::ExitGame);
    commands.spawn((
        container,
        StateScoped(AppState::MainMenu),
        children![start, exit],
    ));
}

fn button(button_type: ButtonType) -> impl Bundle {
    let text = match button_type {
        ButtonType::StartGame => "Start Game!",
        ButtonType::ExitGame => "Exit Game",
    };
    (
        Button,
        button_type,
        BorderColor::from(Color::BLACK),
        Node {
            width: Val::Px(600.),
            height: Val::Px(100.),
            border: UiRect::all(Val::Px(1000.)),
            ..default()
        },
        Text::new(text),
        TextColor(Color::BLACK),
    )
}

#[derive(Component, Clone, Copy)]
enum ButtonType {
    StartGame,
    ExitGame,
}

const NORMAL_BUTTON: Color = Color::srgb(0.4, 0.4, 0.4);
const HOVER_BUTTON: Color = Color::srgb(0.6, 0.6, 0.6);

#[derive(Event)]
struct ClickEvent(ButtonType);

#[hot]
fn button_system(
    mut q_interactions: Query<
        (&Interaction, &ButtonType, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut ev_click: EventWriter<ClickEvent>,
) {
    for (interaction, button_type, mut color) in q_interactions.iter_mut() {
        match *interaction {
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVER_BUTTON.into();
            }
            Interaction::Pressed => {
                *color = HOVER_BUTTON.into();
                ev_click.write(ClickEvent(*button_type));
            }
        }
    }
}

fn handle_click(
    mut ev_click: EventReader<ClickEvent>,
    mut next_state: ResMut<NextState<AppState>>,
    mut ev_exit: EventWriter<AppExit>,
) {
    for click_type in ev_click.read() {
        match click_type.0 {
            ButtonType::StartGame => next_state.set(AppState::InGame),
            ButtonType::ExitGame => {
                ev_exit.write(AppExit::Success);
            }
        }
    }
}
