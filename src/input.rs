use super::map::MapGenEvent;
use bevy::{
    input::Input,
    math::Vec3,
    prelude::{
        App, Camera, Commands, EventWriter, KeyCode, OrthographicCameraBundle, Plugin, Query, Res,
        ResMut, Transform, With,
    },
};
use serde::{Deserialize, Serialize};
use std::{fs, i32, ops::AddAssign};

#[derive(Serialize, Deserialize)]
struct Config {
    regen_map: KeyCode,
    zoom_in: KeyCode,
    zoom_out: KeyCode,
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
        app.insert_resource(Zoom::default());
        app.add_startup_system(create_camera);
        app.add_system(regenerate_map);
        app.add_system(update_camera);
    }

    fn name(&self) -> &str {
        "Fndg::Input"
    }
}

struct Zoom {
    zoom: i32,
    scale: f32,
}

impl Zoom {
    fn new(zoom: i32) -> Self {
        const MIN: i32 = 12;
        const MAX: i32 = 28;
        const BASE: f32 = 1.2;
        let zoom = zoom.clamp(MIN, MAX);
        Self {
            zoom,
            scale: BASE.powi(-zoom),
        }
    }

    fn set_scale(&self, scale: &mut Vec3) {
        scale.x = self.scale;
        scale.y = self.scale;
    }
}

impl AddAssign<i32> for Zoom {
    fn add_assign(&mut self, rhs: i32) {
        let result = Self::new(self.zoom + rhs);
        self.zoom = result.zoom;
        self.scale = result.scale;
    }
}

impl Default for Zoom {
    fn default() -> Self {
        const INITIAL: i32 = 16;
        Self::new(INITIAL)
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

fn create_camera(mut commands: Commands, zoom: Res<Zoom>) {
    let mut camera = OrthographicCameraBundle::new_2d();
    zoom.set_scale(&mut camera.transform.scale);
    commands.spawn_bundle(camera);
}

fn update_camera(
    keys: Res<Input<KeyCode>>,
    config: Res<Config>,
    mut zoom: ResMut<Zoom>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    if let Some(mut camera) = query.iter_mut().next() {
        let change =
            keys.just_pressed(config.zoom_in) as i32 - keys.just_pressed(config.zoom_out) as i32;
        if change != 0 {
            *zoom += change;
            zoom.set_scale(&mut camera.scale);
        }
    }
}
