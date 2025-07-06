#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use avian2d::prelude::*;
use bevy::{prelude::*, window::PresentMode};
use bevy_enhanced_input::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_seedling::prelude::*;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;
use no_mouth::{
    general_movement::*,
    robot::{
        enemy::add_enemy,
        health::*,
        player::{
            input::*,
            movement::*,
            weapons::{CooldownFinished, attack::*},
            *,
        },
    },
    *,
};

fn main() {
    let mut debug_plugins: Option<(EguiPlugin, PhysicsDebugPlugin, WorldInspectorPlugin)> = None;
    println!("nst");
    #[cfg(debug_assertions)]
    {
        debug_plugins = Some((
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            PhysicsDebugPlugin::default(),
            WorldInspectorPlugin::new(),
        ));
    };    
    App::new()
        .add_plugins((
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
            YarnSpinnerPlugin::new(),
            ExampleYarnSpinnerDialogueViewPlugin::new(),
            debug_plugins.unwrap()
        ))
        .add_input_context::<NormalMovement>()
        .add_observer(bind)
        .add_observer(move_horizontal)
        .add_observer(jump)
        .add_observer(hold_jump)
        .add_observer(shoot_projectile)
        .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9)))
        .insert_resource(MovementConfig {
            jump: 1400.0,
            hold_jump: 120.0,
            acceleration: 900.0,
        })
        .insert_resource(Gravity(Vec2::NEG_Y * 12000.0))
        .insert_resource(CooldownFinished(true))
        .insert_resource(DoubleJump(true))
        .insert_resource(Direction(0.0))
        .insert_resource(Actionable(true))
        .insert_resource(PhysicsEnabled(true))
        .insert_resource(MouseCoordinates(Vec2::default()))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                update_grounded,
                update_player_health_bar,
                update_mouse_coords,
                weapon_cooldown,
                aim_weapon,
            ),
        )
        .add_systems(Update, move_camera)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(add_camera());
    commands.spawn(add_enemy(&asset_server));
    commands.spawn(add_player(&asset_server));
    commands.spawn(add_floor(&asset_server));
    commands.spawn((HealthBar, Text::default()));
    commands.add_observer(get_hits);
}
