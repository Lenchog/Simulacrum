use crate::{physics::PhysicsPlugin, prelude::*, setup::SetupPlugin, weapons::WeaponPlugin};
use avian2d::prelude::PhysicsDebugPlugin;
use bevy::{
    app::PluginGroupBuilder,
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
    render::diagnostic::RenderDiagnosticsPlugin,
};

use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use iyes_perf_ui::prelude::*;
pub struct DebugPluginGroup;

impl PluginGroup for DebugPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(EguiPlugin::default())
            .add(PhysicsDebugPlugin::default())
            .add(WorldInspectorPlugin::new())
            .add(FrameTimeDiagnosticsPlugin::default())
            .add(EntityCountDiagnosticsPlugin)
            .add(SystemInformationDiagnosticsPlugin)
            .add(RenderDiagnosticsPlugin)
            .add(PerfUiPlugin)
    }
}
pub struct MyPluginGroup;

impl PluginGroup for MyPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(InputPlugin)
            .add(SetupPlugin)
            .add(PhysicsPlugin)
            .add(MyLdtkPlugin)
            .add(MovementPlugin)
            .add(WeaponPlugin)
            .add(CollectablePlugin)
            .add(CollisionPlugin)
            .add(CameraPlugin)
            .add(UIPlugin)
            .add(bevy_framepace::FramepacePlugin)
            .add(SeedlingPlugin::default())
            .add(TraumaPlugin)
            .add(Light2dPlugin)
            .add(SimpleSubsecondPlugin::default())
    }
}
