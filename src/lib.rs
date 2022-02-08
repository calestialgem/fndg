mod input;
mod map;

use bevy::prelude::*;
use input::InputPlugin;
use map::MapPlugin;

/// Main Bevy plugin of the game, Founding.
pub struct Fndg;

impl PluginGroup for Fndg {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(MapPlugin);
        group.add(InputPlugin);
    }
}

/// A player or an AI.
#[derive(Component)]
struct Nation {
    name: String,
}
