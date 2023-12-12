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

    const fn get(t1: Self, t2: Self, t3: Self) -> Self {
        match (t1, t2, t3) {
            (Self::Trap, Self::Trap | Self::Safe, Self::Safe)
            | (Self::Safe, Self::Trap | Self::Safe, Self::Trap) => Self::Trap,

            _ => Self::Safe,
        }
    }
}

fn main() {
    let mut row: [Tile; 100] = include_str!(r"..\..\input\day18.txt")
        .trim()
        .chars()
        .map(Tile::parse)
        .collect_vec()
        .try_into()
        .ok()
        .unwrap();

    let mut count = row.iter().filter(|tile| matches!(tile, Tile::Safe)).count();

    for _ in 1..400_000 {
        let prev_row = row;

        row[0] = Tile::get(Tile::Safe, prev_row[0], prev_row[1]);
        row[99] = Tile::get(prev_row[98], prev_row[99], Tile::Safe);

        for j in 1..99 {
            row[j] = Tile::get(prev_row[j - 1], prev_row[j], prev_row[j + 1]);
        }

        count += row.iter().filter(|tile| matches!(tile, Tile::Safe)).count();
    }

    dbg!(count);
}
