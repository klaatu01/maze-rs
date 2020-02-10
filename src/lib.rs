mod types;
use types::cell::Cell;
use types::grid::Grid;

pub fn create_grid(x: usize, y: usize) -> Grid<Cell> {
    let grid: Grid<Cell> = Grid::<Cell>::new(x, y);
    grid.print_grid();
    grid
}
