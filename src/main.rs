#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Stable on latest versions, but bevy_lint is old so needs this
#![feature(let_chains)]

use crate::{
    camera::{add_camera, move_camera},
    general_movement::*,
    mouse::{MouseCoordinates, update_mouse_coords},
    robot::{
        enemy::{Enemy, EnemyBundle, add_enemy},
        hits::*,
        player::{input::*, movement::*, weapons::EquipEvent, *},
        ui::*,
    },
    wall::WallBundle,
    weapons::{SelectedHand, attack::*, *},
};
use avian2d::prelude::*;
use bevy::{
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
    render::diagnostic::RenderDiagnosticsPlugin,
    window::PresentMode,
};
use bevy_ecs_ldtk::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_seedling::prelude::*;
use bevy_simple_subsecond_system::prelude::*;
use bevy_trauma_shake::TraumaPlugin;
use iyes_perf_ui::prelude::*;

mod camera;
mod general_movement;
mod mouse;
mod robot;
mod wall;

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
        SimpleSubsecondPlugin::default(),
    ));
    #[cfg(debug_assertions)]
    {
        app.add_plugins((
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            PhysicsDebugPlugin::default(),
            WorldInspectorPlugin::new(),
            FrameTimeDiagnosticsPlugin::default(),
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            RenderDiagnosticsPlugin,
            PerfUiPlugin,
        ));
    }
    app.add_input_context::<NormalMovement>()
        .add_observer(bind)
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
        .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9)))
        .insert_resource(MovementConfig {
            jump: 1400.0,
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
        .add_event::<EquipEvent>()
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<EnemyBundle>("Enemy")
        .register_ldtk_int_cell::<WallBundle>(1)
        .add_observer(setup_player)
        .add_observer(setup_enemy)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                update_grounded,
                update_ui,
                update_mouse_coords,
                update_dash_timer,
                aim_weapon,
                weapon_cooldown,
                swing_weapon,
                got_hit,
                equip_weapon,
                hit_something,
            ),
        )
        .add_systems(Update, move_camera)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
