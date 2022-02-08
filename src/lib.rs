use bevy::prelude::*;
use map::MapPlugin;

mod map;

/// Main Bevy plugin of the game, Founding.
pub struct Fndg;

impl PluginGroup for Fndg {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(MapPlugin);
    }
}

/// A player or an AI.
#[derive(Component)]
struct Nation {
    name: String,
}
