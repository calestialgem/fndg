mod input;
mod map;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    log::{Level, LogSettings},
    prelude::{App, Component, Plugin, PluginGroup},
    window::{WindowDescriptor, WindowMode},
};
use input::InputPlugin;
use map::MapPlugin;

/// Main Bevy plugin of the game, Founding.
pub struct Fndg;

impl PluginGroup for Fndg {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(LogDiagnosticsPlugin::default());
        group.add(FrameTimeDiagnosticsPlugin::default());
        group.add(MainPlugin);
        group.add(MapPlugin);
        group.add(InputPlugin);
    }
}

struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            width: 1280.0,
            height: 720.0,
            title: String::from("Founding"),
            vsync: true,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        });
    }

    fn name(&self) -> &str {
        "Fndg::Main"
    }
}

struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LogSettings {
            filter: String::from("wgpu=warn,bevy_ecs=info"),
            level: Level::DEBUG,
        });
    }

    fn name(&self) -> &str {
        "Fndg::Debug"
    }
}

/// A player or an AI.
#[derive(Component)]
struct Nation {
    name: String,
}
