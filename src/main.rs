#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::setup::MainSetup;
use bevy::prelude::*;
use bevy::window::PresentMode;

mod camera;
mod collectable;
mod ldtk;
mod mouse;
mod physics;
mod plugins;
pub mod prelude;
mod robot;
mod setup;
mod weapons;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
            MainSetup,
        ))
        .run()
}
