/// Renders the game.
pub mod render;

use min_timer::{Now, Stt};
use std::ops::*;

/// State of the game.
pub struct Fndg;

impl Mul<f64> for Fndg {
    type Output = Fndg;

    fn mul(self, rhs: f64) -> Self::Output {
        Self
    }
}

impl Add for Fndg {
    type Output = Fndg;

    fn add(self, rhs: Self) -> Self::Output {
        Self
    }
}

impl Clone for Fndg {
    fn clone(&self) -> Self {
        Self
    }
}

impl Copy for Fndg {}

impl Default for Fndg {
    fn default() -> Self {
        Self
    }
}

impl<T: Now> Stt<T> for Fndg {
    fn init(&mut self, _: &mut min_timer::Hrt<T>, timer: min_timer::Timer<T>) {
        println!("Initialization is complete! Took {}", timer);
    }

    fn update(&mut self, hrt: &mut min_timer::Hrt<T>) {}

    fn sec(&mut self, hrt: &mut min_timer::Hrt<T>) {
        println!(
            "Tick Rate: {} Frame Rate: {}",
            hrt.ticks().avg_rate(),
            hrt.frames().avg_rate()
        );
    }
}
