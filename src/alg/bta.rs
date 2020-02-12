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
    grid.index(1, 1).val.set(0);
    for x in 1..grid.x - 1 {
        for y in 1..grid.y - 1 {
            let c = grid.index(x, y);
            let n_or_e = rng.rand_range(0, 1);
            match n_or_e {
                0 => match grid.north(c) {
                    Some(chunk) => c.val.set(0),
                    _ => (),
                },
                1 => match grid.east(c) {
                    Some(chunk) => c.val.set(0),
                    _ => (),
                },
                _ => (),
            }
        }
    }
}
