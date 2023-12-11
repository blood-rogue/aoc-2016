use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Disc {
    num_slots: usize,
    initial_slot: usize,
}

fn main() {
    let disc_pattern =
        Regex::new(r"^Disc #(\d) has (\d+) positions; at time=0, it is at position (\d)\.$")
            .unwrap();

    let discs = include_str!(r"..\..\input\day15.txt")
        .lines()
        .map(|line| {
            let captures = disc_pattern.captures(line).unwrap();

            Disc {
                num_slots: captures[2].parse().unwrap(),
                initial_slot: captures[3].parse().unwrap(),
            }
        })
        .collect_vec();

    'outer: for time in 0.. {
        for (idx, disc) in discs.iter().enumerate() {
            if (time + idx + 1 + disc.initial_slot) % disc.num_slots > 0 {
                continue 'outer;
            }
        }

        dbg!(time);
        break;
    }
}
