mod alg;
mod disp;
mod types;
use alg::bta::bta;
pub use disp::ascii::format;
pub use types::chunk::Chunk;
pub use types::grid::Grid;

pub fn create_grid(x: usize, y: usize) -> Grid<Chunk> {
    let grid: Grid<Chunk> = Grid::<Chunk>::new(x, y);
    grid.fill(1);
    grid
}

pub fn perform_bta(grid: &Grid<Chunk>) -> () {
    bta(grid);
    println!("{}", format(grid));
}
