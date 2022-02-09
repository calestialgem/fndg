use bevy::{prelude::App, DefaultPlugins};
use fndg::Fndg;

fn main() {
    App::new()
        .add_plugins(Fndg)
        .add_plugins(DefaultPlugins)
        .run();
}
