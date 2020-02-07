mod types;
use types::cell::Cell;
use types::grid::Grid;

fn main() {
    let g: Grid<Cell> = Grid::<Cell>::new(10, 10);
    for i in 0..g.x {
        for j in 0..g.y {
            let c = &g.cells[i][j];
            println!("{}", c.to_string())
        }
    }
}
