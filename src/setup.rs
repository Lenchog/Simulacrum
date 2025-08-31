use std::num::NonZero;

#[cfg(debug_assertions)]
use crate::plugins::DebugPluginGroup;
use crate::{plugins::MyPluginGroup, prelude::*};
use bevy_yarnspinner::prelude::*;
use iyes_perf_ui::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
#[states(scoped_entities)]
pub enum AppState {
    #[default]
    MainMenu,
    Intro,
    InGame,
}

pub struct MainSetupPlugin;

impl Plugin for MainSetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins(MyPluginGroup)
            .add_systems(Startup, main_setup)
            .add_systems(OnEnter(AppState::Intro), start_intro)
            .add_systems(OnEnter(AppState::InGame), start_game)
            .add_systems(OnEnter(AppState::MainMenu), main_menu)
            .add_systems(Update, setup_dialogue.run_if(resource_added::<YarnProject>))
            .add_systems(FixedUpdate, switch_state);
        #[cfg(debug_assertions)]
        app.add_plugins(DebugPluginGroup);
    }
}

fn main_setup(mut commands: Commands, mut time: ResMut<Time<Fixed>>) {
    time.set_timestep_hz(64.0);
    commands.add_observer(bind);
    #[cfg(debug_assertions)]
    commands.spawn(PerfUiDefaultEntries::default());
}

pub fn start_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("main.ldtk").into(),
            ..Default::default()
        },
        StateScoped(AppState::InGame),
    ));
    commands.add_observer(setup_player);
    commands.add_observer(setup_enemy);
    commands.spawn(HealthBar);
    commands.spawn(EnergyBar);
    commands.spawn(SamplePlayer::new(asset_server.load("audio/placeholder_music.wav")).looping());
    commands.add_observer(get_hits);
}

fn switch_state(
    mut next_state: ResMut<NextState<AppState>>,
    current_state: Res<State<AppState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Enter) {
        next_state.set(match current_state.get() {
            AppState::MainMenu => AppState::Intro,
            AppState::Intro => AppState::InGame,
            AppState::InGame => AppState::MainMenu,
        });
    }
}

fn setup_player(trigger: Trigger<OnAdd, Player>, mut commands: Commands) {
    commands.entity(trigger.target()).insert(add_player());
}

fn setup_enemy(trigger: Trigger<OnAdd, Enemy>, mut commands: Commands) {
    commands.entity(trigger.target()).insert(add_enemy());
}

fn setup_dialogue(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);
    dialogue_runner
        .commands_mut()
        .add_command("quit", commands.register_system(yarn_quit))
        .add_command("start_game", commands.register_system(yarn_start_game));
    commands.spawn(dialogue_runner);
}

fn start_intro(mut q_dialogue_runner: Single<&mut DialogueRunner>) {
    q_dialogue_runner.start_node("Intro");
}

fn yarn_quit(_: In<()>, mut ev_exit: EventWriter<AppExit>) {
    ev_exit.write(AppExit::Error(
        NonZero::new(42).expect("Exit code non-zero"),
    ));
}

fn yarn_start_game(_: In<()>, mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::InGame)
}
