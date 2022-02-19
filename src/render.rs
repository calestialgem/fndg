use min_timer::{Now, Render};

use crate::Fndg;

/// Render of the game.
pub struct FndgRender;

impl Default for FndgRender {
    fn default() -> Self {
        Self
    }
}

impl<T: Now> Render<T, Fndg> for FndgRender {
    fn render(&mut self, hrt: &min_timer::Hrt<T>, stt: &Fndg) {}
}
