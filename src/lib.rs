mod alg;
mod disp;
mod types;
use alg::bta::bta;
pub use alg::Algorithm;
pub use disp::ascii::format;
pub use types::chunk::Chunk;
pub use types::grid::Grid;

pub fn create_grid(x: usize, y: usize) -> Grid<Chunk> {
    let grid: Grid<Chunk> = Grid::<Chunk>::new(x, y);
    grid.fill(1);
    grid
}

pub fn perform_alg(grid: &Grid<Chunk>, algorithm: Algorithm) -> () {
    match algorithm {
        Algorithm::Binary => bta(grid),
        _ => (),
    }
    println!("{}", format(grid));
}

pub fn parse_algorithm(algorithm: String) -> Option<Algorithm> {
    match algorithm.to_lowercase().as_str() {
        "binary" | "bin" | "bta" => Some(Algorithm::Binary),
        _ => None,
    }
}
