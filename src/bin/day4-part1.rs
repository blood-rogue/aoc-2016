use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

fn main() {
    let sum = include_str!(r"..\..\input\day4.txt")
        .lines()
        .filter_map(|line| {
            let (data, checksum) = line[..line.len() - 1].split_once('[').unwrap();

            let mut parts = data.split('-').collect_vec();

            let sector_id = parts.pop().unwrap();
            let data = parts.concat();

            checksum_valid(&data, checksum).then_some(sector_id.parse::<usize>().unwrap())
        })
        .sum::<usize>();

    dbg!(sum);
}

fn checksum_valid(encrypted: &str, checksum: &str) -> bool {
    let mut chars = HashMap::new();

    for ch in encrypted.chars() {
        chars.entry(ch).and_modify(|v| *v += 1).or_insert(1);
    }

    let calculated = chars
        .iter()
        .sorted_by(|&(ch1, count1), &(ch2, count2)| match count2.cmp(count1) {
            ordering @ (Ordering::Less | Ordering::Greater) => ordering,
            Ordering::Equal => ch1.cmp(ch2),
        })
        .map(|(ch, _)| *ch)
        .take(5)
        .collect::<String>();

    calculated == checksum
}
