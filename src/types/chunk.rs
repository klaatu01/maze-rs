use super::grid::Dir;

#[derive(Debug, Clone)]
pub struct Chunk {
    y: usize,
    x: usize,
    pub val: std::cell::Cell<usize>,
    pub north: std::cell::Cell<bool>,
    pub east: std::cell::Cell<bool>,
    pub south: std::cell::Cell<bool>,
    pub west: std::cell::Cell<bool>,
}

impl Chunk {
    pub fn new(x: usize, y: usize, val: usize) -> Chunk {
        Chunk {
            x: x,
            y: y,
            val: std::cell::Cell::new(val),
            north: std::cell::Cell::new(false),
            east: std::cell::Cell::new(false),
            south: std::cell::Cell::new(false),
            west: std::cell::Cell::new(false),
        }
    }

    pub fn to_string(&self) -> String {
        "   ".to_string()
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn link(&self, dir: Dir) -> () {
        match dir {
            Dir::North => self.north.set(true),
            Dir::South => self.south.set(true),
            Dir::East => self.east.set(true),
            Dir::West => self.west.set(true),
        }
    }
}
