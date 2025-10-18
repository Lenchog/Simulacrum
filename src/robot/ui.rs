use bevy_yarnspinner::prelude::DialogueRunner;

use crate::{prelude::*, setup::AppState};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (update_ui, button_system, handle_click))
            .add_systems(OnEnter(AppState::Intro), skip_ui)
            .add_message::<ClickMessage>();
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

fn update_ui(
    q_health_bar: Single<&mut Text, (With<HealthBar>, Without<EnergyBar>)>,
    q_energy_bar: Single<&mut Text, With<EnergyBar>>,
    q_player: Single<(&Health, &Energy), With<Player>>,
) {
    let (health, energy) = q_player.into_inner();
    *q_health_bar.into_inner() = Text::new(format!("Health: {}", health.0));
    *q_energy_bar.into_inner() = Text::new(format!("Energy: {}", energy.0));
}

pub fn main_menu(commands: Commands, current_state: ResMut<State<AppState>>) {
    menu(
        "SIMULACRUM",
        &[ButtonType::StartGame, ButtonType::ExitGame],
        commands,
        current_state.get().clone(),
    );
}

pub fn menu(text: &str, buttons: &[ButtonType], mut commands: Commands, current_state: AppState) {
    let logo = (
        Text::new(text),
        TextLayout::new_with_justify(Justify::Center),
        TextFont::from_font_size(128.0),
        Node {
            width: Val::Percent(80.0),
            height: Val::Auto,
            position_type: PositionType::Absolute,
            padding: UiRect::all(Val::Px(100.0)).with_top(Val::Px(300.0)),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            ..default()
        },
    );
    let container = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Start,
        align_items: AlignItems::Center,
        padding: UiRect::bottom(Val::Px(300.0)),
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(100.0),
        ..default()
    };
    let menu = commands
        .spawn((container, children![logo], DespawnOnExit(current_state)))
        .id();
    let button_container = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::End,
        align_items: AlignItems::Center,
        padding: UiRect::bottom(Val::Px(50.0)),
        flex_direction: FlexDirection::Column,
        position_type: PositionType::Absolute,
        row_gap: Val::Px(15.0),
        ..default()
    };
    let menu_buttons = commands.spawn((button_container, ChildOf(menu))).id();
    for button in buttons {
        commands.spawn((build_button(*button), ChildOf(menu_buttons)));
    }
}

fn build_button(button_type: ButtonType) -> impl Bundle {
    let text = match button_type {
        ButtonType::StartGame => "Start Game!",
        ButtonType::ExitGame => "Exit Game",
        ButtonType::MainMenu => "Quit to Main Menu",
        ButtonType::SkipDialogue => "Skip",
    };
    let width = Val::Percent(match button_type {
        ButtonType::SkipDialogue => 30.0,
        _ => 70.0,
    });
    (
        Button,
        button_type,
        BorderColor::from(Color::BLACK),
        Node {
            width,
            height: Val::Px(120.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        TextLayout::new_with_justify(Justify::Center),
        Text::new(text),
        TextColor(Color::BLACK),
    )
}

#[derive(Component, Clone, Copy)]
pub enum ButtonType {
    StartGame,
    MainMenu,
    ExitGame,
    SkipDialogue,
}

const NORMAL_BUTTON: Color = Color::srgb(0.4, 0.4, 0.4);
const HOVER_BUTTON: Color = Color::srgb(0.6, 0.6, 0.6);

#[derive(Message)]
struct ClickMessage(ButtonType);

fn button_system(
    mut q_interactions: Query<
        (&Interaction, &ButtonType, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut ev_click: MessageWriter<ClickMessage>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for (interaction, button_type, mut color) in q_interactions.iter_mut() {
        match *interaction {
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVER_BUTTON.into();
                commands.spawn(SamplePlayer::new(asset_server.load("audio/Bleep_06.wav")));
            }
            Interaction::Pressed => {
                *color = HOVER_BUTTON.into();
                commands.spawn(SamplePlayer::new(asset_server.load("audio/Bleep_01.wav")));
                ev_click.write(ClickMessage(*button_type));
            }
        }
    }
}

fn handle_click(
    mut ev_click: MessageReader<ClickMessage>,
    mut next_state: ResMut<NextState<AppState>>,
    mut ev_exit: MessageWriter<AppExit>,
    mut q_dialogue_runner: Single<&mut DialogueRunner>,
) {
    for click_type in ev_click.read() {
        next_state.set(match click_type.0 {
            ButtonType::StartGame => AppState::Intro,
            ButtonType::MainMenu => AppState::MainMenu,
            ButtonType::ExitGame => {
                ev_exit.write(AppExit::Success);
                AppState::Unreachable
            }
            ButtonType::SkipDialogue => {
                q_dialogue_runner.stop();
                AppState::InGame
            }
        })
    }
}

fn skip_ui(mut commands: Commands) {
    let container = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::End,
        padding: UiRect::all(Val::Px(20.0)),
        ..default()
    };
    commands.spawn((
        children![build_button(ButtonType::SkipDialogue)],
        container,
        DespawnOnExit(AppState::Intro),
    ));
}
