/*pub trait Cell {
    fn new(x: u32, y: u32) -> Self;
    fn to_string(&self) -> String;
}*/
#[derive(Debug, Copy, Clone)]
pub struct Cell {
    y: usize,
    x: usize,
    pub val: usize,
}

impl Cell {
    pub fn new(x: usize, y: usize, val: usize) -> Cell {
        Cell {
            x: x,
            y: y,
            val: val,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("x:{},y:{},v:{}", self.x, self.y, self.val);
    }
}
