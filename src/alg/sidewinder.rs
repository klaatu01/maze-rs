use super::super::Chunk;
use super::super::Grid;
use super::bta::Rand;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn sidewinder(grid: &Grid<Chunk>) -> () {
    let mut rng = Rand::new(0);
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(t) => rng = Rand::new(t.as_millis() as u32),
        _ => (),
    }
    for y in 0..grid.y {
        let mut run = Vec::<&Chunk>::new();
        for x in 0..grid.x {
            let chunk = &grid.index(x, y);
            run.push(chunk);

            let is_north = y == grid.y - 1;
            let is_east = x == grid.x - 1;

            let should_close = is_east || (!is_north && rng.rand_range(0, 1) == 0);

            if should_close {
                let member = run[rng.rand_range(0, (run.len() - 1) as i32) as usize];
                match grid.north(member) {
                    Some(c) => grid.link(member, c),
                    None => (),
                }
                run.clear();
            } else {
                match grid.east(chunk) {
                    Some(c) => grid.link(chunk, c),
                    None => (),
                }
            }
        }
    }
}
