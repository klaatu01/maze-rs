mod alg;
mod disp;
mod types;
use alg::bta::bta;
use alg::sidewinder::sidewinder;
pub use alg::Algorithm;
pub use disp::ascii::format;
pub use types::chunk::Chunk;
pub use types::grid::Grid;

pub fn create_grid(x: usize, y: usize) -> Grid<Chunk> {
    let grid: Grid<Chunk> = Grid::<Chunk>::new(x, y);
    grid.fill(1);
    grid
}

pub fn get_algorithm_from_str(algorithm: String) -> fn(&Grid<Chunk>) -> () {
    let a = parse_algorithm(algorithm);
    get_algorithm(a)
}

pub fn get_algorithm(algorithm: Algorithm) -> fn(&Grid<Chunk>) -> () {
    let a = match algorithm {
        Algorithm::Binary => bta,
        Algorithm::Sidewinder => sidewinder,
        _ => panic!("Algorithm Not Implemented"),
    };
    a
}

pub fn parse_algorithm(algorithm: String) -> Algorithm {
    match algorithm.to_lowercase().as_str() {
        "binary" | "bin" | "bta" => Algorithm::Binary,
        "sidewinder" | "side" | "sw" => Algorithm::Sidewinder,
        _ => panic!("Could not parse {} as algorithm", algorithm.clone()),
    }
}
