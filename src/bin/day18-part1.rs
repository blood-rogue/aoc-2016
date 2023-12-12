use itertools::Itertools;

#[derive(Clone, Copy)]
enum Tile {
    Safe,
    Trap,
}

impl Tile {
    const fn parse(s: char) -> Self {
        if s == '.' {
            Self::Safe
        } else {
            Self::Trap
        }
    }

    fn set_tile(&mut self, t1: Self, t2: Self, t3: Self) {
        match (t1, t2, t3) {
            (Self::Trap, Self::Trap | Self::Safe, Self::Safe)
            | (Self::Safe, Self::Trap | Self::Safe, Self::Trap) => *self = Self::Trap,

            _ => {}
        }
    }
}

fn main() {
    let mut grid = [[Tile::Safe; 100]; 40];

    grid[0] = include_str!(r"..\..\input\day18.txt")
        .trim()
        .chars()
        .map(Tile::parse)
        .collect_vec()
        .try_into()
        .ok()
        .unwrap();

    for i in 1..40 {
        let prev_row = grid[i - 1];

        grid[i][0].set_tile(Tile::Safe, prev_row[0], prev_row[1]);
        grid[i][99].set_tile(prev_row[98], prev_row[99], Tile::Safe);

        for j in 1..99 {
            grid[i][j].set_tile(prev_row[j - 1], prev_row[j], prev_row[j + 1]);
        }
    }

    let count = grid
        .iter()
        .flatten()
        .filter(|tile| matches!(tile, Tile::Safe))
        .count();

    dbg!(count);
}
