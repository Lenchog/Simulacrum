#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Stable on latest versions, but bevy_lint is old so needs this
#![feature(let_chains)]

pub mod prelude {
    pub use crate::{
        camera::*,
        general_movement::*,
        input::*,
        mouse::*,
        player::movement::*,
        robot::{enemy::*, hits::*, player::*, ui::*, *},
        wall::*,
    };
    pub use avian2d::{math::PI, prelude::*};
    pub use bevy::prelude::*;
    pub use bevy_ecs_ldtk::prelude::*;
    pub use bevy_light_2d::prelude::*;
    pub use bevy_seedling::prelude::*;
    pub use bevy_simple_subsecond_system::prelude::*;
    pub use bevy_trauma_shake::prelude::*;
    pub use std::time::Duration;
}
use crate::{prelude::*, weapons::prelude::*};
use bevy::{
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    render::diagnostic::RenderDiagnosticsPlugin,
    window::PresentMode,
};
use bevy_enhanced_input::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use iyes_perf_ui::prelude::*;

#[derive(Component, Default)]
pub struct Battery;

#[derive(LdtkEntity, Default, Bundle)]
pub struct BatteryBundle {
    battery: Battery,
    #[sprite_sheet]
    sprite: Sprite,
}

mod camera;
mod general_movement;
mod mouse;
mod robot;
mod wall;
mod weapons;

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }),
        PhysicsPlugins::default()
            .with_length_unit(20.0)
            .set(PhysicsInterpolationPlugin::interpolate_all()),
        bevy_framepace::FramepacePlugin,
        EnhancedInputPlugin,
        SeedlingPlugin::default(),
        LdtkPlugin,
        TraumaPlugin,
        Light2dPlugin,
        SimpleSubsecondPlugin::default(),
    ));
    #[cfg(debug_assertions)]
    {
        app.add_plugins((
            EguiPlugin::default(),
            PhysicsDebugPlugin::default(),
            WorldInspectorPlugin::new(),
            FrameTimeDiagnosticsPlugin::default(),
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            RenderDiagnosticsPlugin,
            PerfUiPlugin,
        ));
    }
    app.add_input_context::<Player>()
        .add_observer(move_horizontal)
        .add_observer(jump)
        .add_observer(dash)
        .add_observer(hold_jump)
        .add_observer(attack)
        .add_observer(select_left)
        .add_observer(select_right)
        .add_observer(equip_sword)
        .add_observer(equip_gun)
        .add_observer(equip_fast_gun)
        .add_observer(equip_power_gun)
        .add_observer(equip_rocket_launcher)
        .add_observer(equip_grappling_hook)
        .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9)))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            ..default()
        })
        .insert_resource(MovementConfig {
            jump: 2000.0,
            dash: 3000.0,
            hold_jump: 120.0,
            speed: 900.0,
        })
        .insert_resource(EquippedWeapons {
            left: None,
            right: None,
        })
        .insert_resource(Gravity(Vec2::NEG_Y * 12000.0))
        .insert_resource(MouseCoordinates(Vec2::default()))
        .insert_resource(LevelSelection::index(0))
        .insert_resource(MaxEnergy(100))
        .insert_resource(SelectedHand::Left)
        .insert_resource(EquippedWeapons {
            left: None,
            right: None,
        })
        .add_event::<HitEvent>()
        .add_event::<Unhook>()
        .add_event::<EquipEvent>()
        .add_event::<ShootEvent>()
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<BatteryBundle>("Battery")
        .register_ldtk_entity::<EnemyBundle>("Enemy")
        .register_ldtk_int_cell::<WallBundle>(1)
        .register_ldtk_int_cell::<SpikeBundle>(2)
        .add_observer(setup_player)
        .add_observer(setup_enemy)
        .add_observer(bind)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                update_grounded,
                update_ui,
                update_mouse_coords,
                update_dash_timer,
                update_explosion_timer,
                aim_weapon,
                weapon_cooldown,
                swing_weapon,
                got_hit,
                equip_weapon,
                hit_something,
                handle_grapple_hook,
                retract_hook,
                unhook,
                shoot,
                level_selection_follow_player,
                update_grid_coords,
            ),
        )
        .add_systems(Update, move_camera)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut time: ResMut<Time<Fixed>>) {
    time.set_timestep_hz(128.0);
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("main.ldtk").into(),
        ..Default::default()
    });
    commands.spawn(add_camera());
    commands.spawn(HealthBar);
    commands.spawn(EnergyBar);
    commands.add_observer(get_hits);
    commands.spawn(PerfUiDefaultEntries::default());
}

fn setup_player(trigger: Trigger<OnAdd, Player>, mut commands: Commands) {
    commands.entity(trigger.target()).insert(add_player());
}

fn setup_enemy(trigger: Trigger<OnAdd, Enemy>, mut commands: Commands) {
    commands.entity(trigger.target()).insert(add_enemy());
}
fn level_selection_follow_player(
    q_player: Single<&GlobalTransform, With<Player>>,
    q_levels: Query<(&LevelIid, &GlobalTransform)>,
    q_ldtk_projects: Single<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut level_selection: ResMut<LevelSelection>,
) {
    for (level_iid, level_transform) in q_levels.iter() {
        let ldtk_project = ldtk_project_assets
            .get(*q_ldtk_projects)
            .expect("ldtk project should be loaded before player is spawned");

        let level = ldtk_project
            .get_raw_level_by_iid(level_iid.get())
            .expect("level should exist in only project");

        let level_bounds = Rect {
            min: Vec2::new(
                level_transform.translation().x,
                level_transform.translation().y,
            ),
            max: Vec2::new(
                level_transform.translation().x + level.px_wid as f32,
                level_transform.translation().y + level.px_hei as f32,
            ),
        };
        if level_bounds.contains(q_player.translation().truncate()) {
            *level_selection = LevelSelection::Iid(level_iid.clone());
        }
    }
}
