use super::cell::Cell;

pub struct Grid<T> {
    pub y: usize,
    pub x: usize,
    pub cells: Vec<T>,
}

impl Grid<Cell> {
    pub fn new(x: usize, y: usize) -> Grid<Cell> {
        let mut grid = Grid {
            x: x,
            y: y,
            cells: Vec::with_capacity(x),
        };
        for i in 0..x {
            for j in 0..y {
                let cell = Cell::new(i, j, 0);
                grid.cells.push(cell);
            }
        }
        grid
    }

    pub fn make_boarder(&self) -> () {
        for i in 0..self.x {
            for j in 0..self.y {
                if i == 0 || i == self.x - 1 || j == 0 || j == self.y - 1 {
                    self.index(i, j).val.set(1);
                }
            }
        }
    }

    pub fn index(&self, x: usize, y: usize) -> &Cell {
        &self.cells[(x * self.y) + y]
    }

    pub fn print_grid(&self) -> () {
        let mut s = String::new();
        for i in 0..self.x {
            for j in 0..self.y {
                let c = self.index(i, j);
                match c.val.get() {
                    1 => s += "X",
                    _ => s += " ",
                }
            }
            s += "\n";
        }
        println!("{}", s)
    }
}
