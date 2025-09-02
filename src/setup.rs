use bevy_hanabi::prelude::*;
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
            .add_systems(Update, setup_dialogue.run_if(resource_added::<YarnProject>));
        #[cfg(debug_assertions)]
        app.add_plugins(DebugPluginGroup)
            .add_systems(FixedUpdate, switch_state);
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
    commands.add_observer(rain);
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

fn rain(
    _: Trigger<OnAdd, Player>,
    mut effects: ResMut<Assets<EffectAsset>>,
    q_camera: Single<Entity, With<Camera>>,
    mut commands: Commands,
) {
    // Define a color gradient from red to transparent black
    let mut gradient = Gradient::new();
    gradient.add_key(
        0.0,
        Vec4::new(72.0 / 255.0, 105. / 255.00, 211. / 255.00, 0.55),
    );
    gradient.add_key(1.0, Vec4::splat(0.));

    // Create a new expression module
    let mut module = Module::default();

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        // radius of the world plus 1.5x room
        radius: module.lit(2880.),
        dimension: ShapeDimension::Volume,
    };

    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = module.lit(3.); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.3, Vec3::new(20.2, 20.02, 1.0));
    size_gradient1.add_key(1.0, Vec3::splat(0.0));

    // Every frame, add a gravity-like acceleration downward
    let accel = module.lit(Vec3::new(0., -2000., 0.));
    let update_accel = AccelModifier::new(accel);

    // Create the effect asset
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        32768,
        SpawnerSettings::rate(3000.0.into()),
        // Move the expression module into the asset
        module,
    )
    .with_name("MyEffect")
    .init(init_pos)
    .init(init_lifetime)
    .update(update_accel)
    .render(SizeOverLifetimeModifier {
        gradient: size_gradient1,
        screen_space_size: false,
    })
    // Render the particles with a color gradient over their
    // lifetime. This maps the gradient key 0 to the particle spawn
    // time, and the gradient key 1 to the particle death (10s).
    .render(ColorOverLifetimeModifier {
        gradient,

        ..default()
    });

    // Insert into the asset system
    let effect_handle = effects.add(effect);
    commands.spawn((ParticleEffect::new(effect_handle), ChildOf(*q_camera)));
}
