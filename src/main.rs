use fndg::{render::FndgRender, Fndg};
use min_gl::{Display, Options};
use min_timer::Hrt;

fn main() {
    let disp = Display::new(
        Options {
            width: 1280,
            height: 720,
            title: "Founding II".into(),
            fullscreen: false,
            decorated: true,
            msaa: Some(16),
            vsync: true,
        },
        |_| {},
    );
    let mut hrt = Hrt::new(20.0, &disp);
    hrt.start::<Fndg, FndgRender>();
}
