use super::super::Chunk;
use super::super::Grid;
extern crate piston_window;
use piston_window::*;

pub struct Display {
    grid: Grid<Chunk>,
    pub window: PistonWindow,
}

impl Display {
    pub fn new(grid: Grid<Chunk>, window: PistonWindow) -> Display {
        Display {
            grid: grid,
            window: window,
        }
    }

    pub fn update(&mut self, event: Event) -> () {
        let grid = &self.grid;
        let width = self.window.size().width;
        let height = self.window.size().height;
        self.window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            for x in 0..grid.x {
                for y in 0..grid.y {
                    rectangle(
                        match grid.index(x, y).val.get() {
                            0 => [0.0, 0.0, 0.0, 1.0],
                            _ => [1.0, 0.0, 0.0, 1.0],
                        },
                        [
                            (20.0 * f64::from(x as i32)),
                            height - 20.0 - (20.0 * f64::from(y as i32)),
                            19.0,
                            19.0,
                        ],
                        context.transform,
                        graphics,
                    );
                }
            }
        });
        ()
    }
}
