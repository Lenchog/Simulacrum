use crate::{physics::PhysicsPlugin, prelude::*, weapons::WeaponPlugin};
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
use bevy_hanabi::HanabiPlugin;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_yarnspinner::prelude::YarnSpinnerPlugin;
use bevy_yarnspinner_example_dialogue_view::ExampleYarnSpinnerDialogueViewPlugin;
use iyes_perf_ui::prelude::*;

#[allow(dead_code)]
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
            .add(YarnSpinnerPlugin::new())
            .add(ExampleYarnSpinnerDialogueViewPlugin::new())
            .add(SimpleSubsecondPlugin::default())
            .add(HanabiPlugin)
    }
}
