use super::super::Chunk;
use super::super::Grid;
use std::time::{SystemTime, UNIX_EPOCH};

const KX: u32 = 123456789;
const KY: u32 = 362436069;
const KZ: u32 = 521288629;
const KW: u32 = 88675123;

pub struct Rand {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl Rand {
    pub fn new(seed: u32) -> Rand {
        Rand {
            x: KX ^ seed,
            y: KY ^ seed,
            z: KZ,
            w: KW,
        }
    }

    // Xorshift 128, taken from German Wikipedia
    pub fn rand(&mut self) -> u32 {
        let t = self.x ^ self.x.wrapping_shl(11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w ^= self.w.wrapping_shr(19) ^ t ^ t.wrapping_shr(8);
        return self.w;
    }

    pub fn rand_range(&mut self, a: i32, b: i32) -> i32 {
        let m = (b - a + 1) as u32;
        return a + (self.rand() % m) as i32;
    }
}

pub fn bta(grid: &Grid<Chunk>) -> () {
    let mut rng = Rand::new(0);
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(t) => rng = Rand::new(t.as_millis() as u32),
        _ => (),
    }
    grid.index(0, 0).val.set(0);
    for x in 0..grid.x {
        for y in 0..grid.y {
            let c = grid.index(x, y);
            let n_or_e = rng.rand_range(0, 1);

            let is_north = y == grid.y - 1;
            let is_east = x == grid.x - 1;
            fn link_if_some(grid: &Grid<Chunk>, chunk: &Chunk, opt_chunk: Option<&Chunk>) -> () {
                match opt_chunk {
                    Some(ch) => grid.link(chunk, ch),
                    None => (),
                };
                ()
            }
            match (is_north, is_east) {
                (false, false) => match n_or_e {
                    0 => link_if_some(grid, c, grid.north(c)),
                    1 => link_if_some(grid, c, grid.east(c)),
                    _ => (),
                },
                (true, false) => link_if_some(grid, c, grid.east(c)),
                (false, true) => link_if_some(grid, c, grid.north(c)),
                (true, true) => (),
            }
        }
    }
}
