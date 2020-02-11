mod types;
use types::chunk::Chunk;
use types::grid::Grid;
mod alg;
use alg::bta::bta;

pub fn create_grid(x: usize, y: usize) -> Grid<Chunk> {
    let grid: Grid<Chunk> = Grid::<Chunk>::new(x, y);
    grid.fill(1);
    grid
}

pub fn perform_bta(grid: &Grid<Chunk>) -> () {
    bta(grid);
}
