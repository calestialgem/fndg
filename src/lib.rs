use bevy::prelude::*;

/// Main Bevy plugin of the game, Founding.
pub struct Fndg;

impl Plugin for Fndg {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_nations);
        app.add_system(greet_nations);
    }

    fn name(&self) -> &str {
        "Founding"
    }
}

/// A player or an AI.
#[derive(Component)]
struct Nation {
    name: String,
}

impl Nation {
    /// Crates with a name from the given `&str`.
    fn new(name: &str) -> Self {
        Nation {
            name: String::from(name),
        }
    }

    /// Prints a simple message for testing the project setup.
    fn greet(&self) {
        println!("I salute you, the great ruler of {}!", self.name);
    }
}

/// Adds hardcoded nations for testing the project setup.
fn add_nations(mut commands: Commands) {
    commands.spawn().insert(Nation::new("Red Hoarders"));
    commands.spawn().insert(Nation::new("Blue Climbers"));
    commands.spawn().insert(Nation::new("Green Writers"));
}

/// Prints a debug message for all the nations.
fn greet_nations(nations: Query<&Nation>) {
    nations.iter().for_each(Nation::greet);
}
