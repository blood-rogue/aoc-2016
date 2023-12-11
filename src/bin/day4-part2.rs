use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

fn main() {
    for line in include_str!(r"..\..\input\day4.txt").lines() {
        let (data, checksum) = line[..line.len() - 1].split_once('[').unwrap();

        let mut parts = data.split('-').collect_vec();

        let sector_id = parts.pop().unwrap().parse::<usize>().unwrap();
        let data = parts.join("-");

        if checksum_valid(&data, checksum) {
            let mut decrypted = Vec::with_capacity(data.len());

            for ch in data.bytes() {
                decrypted.push(if ch == b'-' {
                    ' '
                } else {
                    char::from(((ch - b'a' + u8::try_from(sector_id % 26).unwrap()) % 26) + b'a')
                });
            }

            if decrypted.iter().collect::<String>().contains("north") {
                dbg!(sector_id);
                break;
            }
        }
    }
}

fn checksum_valid(encrypted: &str, checksum: &str) -> bool {
    let mut chars = HashMap::new();

    for ch in encrypted.chars() {
        if ch != '-' {
            chars.entry(ch).and_modify(|v| *v += 1).or_insert(1);
        }
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
