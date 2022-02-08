use bevy::{prelude::App, DefaultPlugins};
use fndg::Fndg;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Fndg)
        .run();
}
