use maze::create_grid;
use maze::perform_bta;

use maze::disp::display::Display;

extern crate piston_window;

use piston_window::*;

fn main() {
    let g = create_grid(10, 10);
    perform_bta(&g);

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut d = Display::new(g, window);

    while let Some(event) = d.window.next() {
        d.update(event);
    }
}
