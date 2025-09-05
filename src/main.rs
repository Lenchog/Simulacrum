use crate::setup::MainSetupPlugin;
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, WgpuSettings};
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
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                }),
            MainSetupPlugin,
        ))
        .run()
}
