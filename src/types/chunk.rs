/*pub trait Cell {
    fn new(x: u32, y: u32) -> Self;
    fn to_string(&self) -> String;
}*/
#[derive(Debug, Clone)]
pub struct Chunk {
    y: usize,
    x: usize,
    pub val: std::cell::Cell<usize>,
}

impl Chunk {
    pub fn new(x: usize, y: usize, val: usize) -> Chunk {
        Chunk {
            x: x,
            y: y,
            val: std::cell::Cell::new(val),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("x:{},y:{},v:{}", self.x, self.y, self.val.get());
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}
