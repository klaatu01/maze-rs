use super::cell::Cell;

pub struct Grid<T> {
    pub y: usize,
    pub x: usize,
    pub cells: Vec<Vec<T>>,
}

impl Grid<Cell> {
    pub fn new(x: usize, y: usize) -> Grid<Cell> {
        let mut grid = Grid {
            x: x,
            y: y,
            cells: Vec::with_capacity(x),
        };
        for i in 0..x {
            let mut row = Vec::with_capacity(y);
            for j in 0..y {
                row.push(Cell::new(i, j));
            }
            grid.cells.push(row)
        }
        grid
    }
}
