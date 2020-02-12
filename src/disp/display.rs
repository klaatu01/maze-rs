use super::super::Chunk;
use super::super::Grid;
extern crate piston_window;
use piston_window::*;

pub struct Display {
    grid: Grid<Chunk>,
    window: PistonWindow,
};

impl Display {
    pub fn new(grid: &Grid<Chunk>, window: &PistonWindow) -> Display {
        Display {
            grid: grid,
            window: window
        }
    }

    pub fn update(&self, event: &Event) -> () {
        self.window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);
        });
        ()
    }
}