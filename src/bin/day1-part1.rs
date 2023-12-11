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

    const fn walk(self, (x, y): (isize, isize), units: isize) -> (isize, isize) {
        match self {
            Self::North => (x + units, y),
            Self::South => (x - units, y),
            Self::East => (x, y + units),
            Self::West => (x, y - units),
        }
    }
}

fn main() {
    let mut curr_dir = Direction::North;
    let mut curr_pos = (0, 0);

    include_str!(r"..\..\input\day1.txt")
        .trim()
        .split(", ")
        .for_each(|instruction| {
            let (dir, units) = instruction.split_at(1);
            curr_dir.turn(dir);

            curr_pos = curr_dir.walk(curr_pos, units.parse().unwrap());
        });

    let distance = curr_pos.0.unsigned_abs() + curr_pos.1.unsigned_abs();

    dbg!(distance);
}
