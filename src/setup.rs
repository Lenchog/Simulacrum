use crate::prelude::*;
use iyes_perf_ui::prelude::*;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(setup_player)
            .add_observer(setup_enemy)
            .add_observer(bind)
            .add_systems(Startup, setup);
    }
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut time: ResMut<Time<Fixed>>) {
    time.set_timestep_hz(64.0);
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("main.ldtk").into(),
        ..Default::default()
    });
    commands.spawn(add_camera());
    commands.spawn(HealthBar);
    commands.spawn(EnergyBar);
    commands.add_observer(get_hits);
    #[cfg(debug_assertions)]
    commands.spawn(PerfUiDefaultEntries::default());
}

fn setup_player(trigger: Trigger<OnAdd, Player>, mut commands: Commands) {
    commands.entity(trigger.target()).insert(add_player());
}

fn setup_enemy(trigger: Trigger<OnAdd, Enemy>, mut commands: Commands) {
    commands.entity(trigger.target()).insert(add_enemy());
}
