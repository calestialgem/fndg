mod input;
mod map;

use bevy::{
    log::{Level, LogSettings},
    prelude::*,
};
use input::InputPlugin;
use map::MapPlugin;

/// Main Bevy plugin of the game, Founding.
pub struct Fndg;

impl PluginGroup for Fndg {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(DebugPlugin);
        group.add(MapPlugin);
        group.add(InputPlugin);
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
