use std::iter;

use super::super::types::chunk::Chunk;
use super::super::types::grid::Dir;
use super::super::types::grid::Grid;

pub fn format(grid: &Grid<Chunk>) -> String {
    let mut res = String::new();
    res += "+";
    res += &iter::repeat("---+").take(grid.x).collect::<String>()[..];
    res += "\n";

    for y in 0..grid.y {
        let mut top = "|".to_string();
        let mut bottom = "+".to_string();

        for x in 0..grid.x {
            let mut chunk = &grid.index(x, y);
            top += &chunk.to_string()[..];
            match grid.linked(chunk, Dir::East) {
                true => top += " ",
                false => top += "|",
            }

            match grid.linked(chunk, Dir::North) {
                true => bottom += "   +",
                false => bottom += "---+",
            }
        }

        res += &top[..];
        res += "\n";

        res += &bottom[..];
        res += "\n";
    }

    return res;
}
