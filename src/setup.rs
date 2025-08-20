#[cfg(debug_assertions)]
use crate::plugins::DebugPluginGroup;
use crate::{plugins::MyPluginGroup, prelude::*};
use iyes_perf_ui::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
#[states(scoped_entities)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}

pub struct MainSetupPlugin;

impl Plugin for MainSetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();
        app.add_plugins(MyPluginGroup);
        #[cfg(debug_assertions)]
        app.add_plugins(DebugPluginGroup);
        app.add_systems(Startup, main_setup);
        app.add_systems(OnEnter(AppState::InGame), start_game);
        app.add_systems(OnEnter(AppState::MainMenu), main_menu);
        app.add_systems(FixedUpdate, switch_state);
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
    commands.add_observer(get_hits);
}

fn switch_state(
    mut next_state: ResMut<NextState<AppState>>,
    current_state: Res<State<AppState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Enter) {
        next_state.set(match current_state.get() {
            AppState::MainMenu => AppState::InGame,
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
