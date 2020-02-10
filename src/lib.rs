mod types;
use types::chunk::Chunk;
use types::grid::Grid;

pub fn create_grid(x: usize, y: usize) -> Grid<Chunk> {
    let grid: Grid<Chunk> = Grid::<Chunk>::new(x, y);
    grid.print_grid();
    grid.make_boarder();
    grid.print_grid();
    grid
}
