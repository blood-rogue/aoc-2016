use std::collections::HashSet;

use itertools::Itertools;

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    const fn turn_left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::East => Self::North,
            Self::West => Self::South,
        }
    }

    const fn turn_right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
        }
    }

    fn turn(&mut self, dir: &str) {
        *self = if dir == "L" {
            self.turn_left()
        } else {
            self.turn_right()
        }
    }

    const fn walk(self, (x, y): (isize, isize)) -> (isize, isize) {
        match self {
            Self::North => (x + 1, y),
            Self::South => (x - 1, y),
            Self::East => (x, y + 1),
            Self::West => (x, y - 1),
        }
    }
}

fn main() {
    let instructions = include_str!(r"..\..\input\day1.txt")
        .trim()
        .split(", ")
        .collect_vec();

    let mut curr_dir = Direction::North;
    let mut curr_pos = (0, 0);

    let mut visited = HashSet::new();

    'outer: for instruction in instructions {
        let (dir, units) = instruction.split_at(1);
        curr_dir.turn(dir);

        for _ in 0..units.parse::<usize>().unwrap() {
            curr_pos = curr_dir.walk(curr_pos);
            if !visited.insert(curr_pos) {
                break 'outer;
            }
        }
    }

    let distance = curr_pos.0.unsigned_abs() + curr_pos.1.unsigned_abs();
    dbg!(distance);
}
