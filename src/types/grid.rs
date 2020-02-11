use super::chunk::Chunk;

pub struct Grid<T> {
    pub y: usize,
    pub x: usize,
    pub chunks: Vec<T>,
}

impl Grid<Chunk> {
    pub fn new(x: usize, y: usize) -> Grid<Chunk> {
        let mut grid = Grid {
            x: x,
            y: y,
            chunks: Vec::with_capacity(x),
        };
        for i in 0..x {
            for j in 0..y {
                let chunk = Chunk::new(i, j, 0);
                grid.chunks.push(chunk);
            }
        }
        grid
    }

    pub fn apply_boarder(&self) -> () {
        for i in 0..self.x {
            for j in 0..self.y {
                if i == 0 || i == self.x - 1 || j == 0 || j == self.y - 1 {
                    self.index(i, j).val.set(1);
                }
            }
        }
    }

    pub fn fill(&self, val: usize) -> () {
        for i in 0..self.x {
            for j in 0..self.y {
                self.index(i, j).val.set(val);
            }
        }
    }

    pub fn index(&self, x: usize, y: usize) -> &Chunk {
        &self.chunks[(x * self.y) + y]
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for i in 0..self.x {
            for j in 0..self.y {
                let c = self.index(i, j);
                match c.val.get() {
                    1 => s += "X ",
                    _ => s += "  ",
                }
            }
            s += "\n";
        }
        s.to_string()
    }

    pub fn north(&self, chunk: &Chunk) -> Option<&Chunk> {
        if chunk.y() + 1 < self.y {
            return Some(self.index(chunk.x(), chunk.y() + 1));
        }
        None
    }

    pub fn east(&self, chunk: &Chunk) -> Option<&Chunk> {
        if chunk.x() + 1 < self.x {
            return Some(self.index(chunk.x() + 1, chunk.y()));
        }
        None
    }
}
