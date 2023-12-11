use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let mut columns = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];

    include_str!(r"..\..\input\day6.txt")
        .lines()
        .for_each(|line| {
            for (i, ch) in line.chars().enumerate() {
                columns[i].push(ch);
            }
        });

    let message = columns
        .map(|col| {
            let mut char_map = HashMap::new();

            for ch in col {
                char_map.entry(ch).and_modify(|v| *v += 1).or_insert(1);
            }

            *char_map
                .iter()
                .sorted_by_key(|(_, count)| *count)
                .last()
                .unwrap()
                .0
        })
        .iter()
        .collect::<String>();

    dbg!(message);
}
