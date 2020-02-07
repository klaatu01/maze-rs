/*pub trait Cell {
    fn new(x: u32, y: u32) -> Self;
    fn to_string(&self) -> String;
}*/

pub struct Cell {
    y: usize,
    x: usize,
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Cell {
        Cell { x: x, y: y }
    }

    pub fn to_string(&self) -> String {
        return format!("x:{},y:{}", self.x, self.y);
    }
}
