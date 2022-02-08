use super::map::MapGenEvent;
use bevy::{
    input::Input,
    prelude::{App, EventWriter, KeyCode, Plugin, Res},
};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Config {
    regen_map: KeyCode,
}

impl Config {
    pub(crate) fn new() -> Self {
        serde_json::from_str(&fs::read_to_string("assets/input.json").unwrap()).unwrap()
    }
}

pub(super) struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Config::new());
        app.add_system(regenerate_map);
    }

    fn name(&self) -> &str {
        "Fndg::Input"
    }
}

fn regenerate_map(
    keys: Res<Input<KeyCode>>,
    config: Res<Config>,
    mut gen_event: EventWriter<MapGenEvent>,
) {
    if keys.just_pressed(config.regen_map) {
        gen_event.send(MapGenEvent);
    }
}
