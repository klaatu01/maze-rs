use super::chunk::Chunk;

pub struct Grid<T> {
    pub y: usize,
    pub x: usize,
    pub chunks: Vec<T>,
}

pub enum Dir {
    North,
    East,
    South,
    West,
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
        for j in 0..self.y {
            for i in 0..self.x {}
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

    pub fn south(&self, chunk: &Chunk) -> Option<&Chunk> {
        if chunk.y() - 1 > 0 {
            return Some(self.index(chunk.x(), chunk.y() - 1));
        }
        None
    }

    pub fn west(&self, chunk: &Chunk) -> Option<&Chunk> {
        if chunk.x() - 1 > 0 {
            return Some(self.index(chunk.x() - 1, chunk.y()));
        }
        None
    }

    pub fn link(&self, a: &Chunk, b: &Chunk) -> () {
        let x_diff: isize = (b.x() - a.x()) as isize;
        let y_diff: isize = (b.y() - a.y()) as isize;
        match (x_diff, y_diff) {
            (0, 1) => {
                a.link(Dir::North);
                b.link(Dir::South);
            }
            (0, -1) => {
                a.link(Dir::South);
                b.link(Dir::North);
            }
            (1, 0) => {
                a.link(Dir::East);
                b.link(Dir::West);
            }
            (-1, 0) => {
                a.link(Dir::West);
                b.link(Dir::East);
            }
            (_, _) => (),
        }
    }

    pub fn linked(&self, cell: &Chunk, direction: Dir) -> bool {
        match direction {
            Dir::North => match self.north(cell) {
                Some(c) => cell.north.get() && c.south.get(),
                _ => false,
            },
            Dir::East => match self.east(cell) {
                Some(c) => cell.east.get() && c.west.get(),
                _ => false,
            },
            Dir::South => match self.south(cell) {
                Some(c) => cell.south.get() && c.north.get(),
                _ => false,
            },
            Dir::West => match self.west(cell) {
                Some(c) => cell.west.get() && c.east.get(),
                _ => false,
            },
            _ => false,
        }
    }
}
