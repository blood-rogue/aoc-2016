use std::collections::HashSet;

fn main() {
    let count = include_str!(r"..\..\input\day7.txt")
        .lines()
        .filter(|line| {
            let mut hypernet_sequence = Vec::new();
            let mut supernet_sequences = Vec::new();

            for part in line.split_inclusive(|ch| ch == '[' || ch == ']') {
                if part.ends_with(']') {
                    hypernet_sequence.push(part.trim_end_matches(']'));
                } else {
                    supernet_sequences.push(part.trim_end_matches('['));
                }
            }

            let mut babs = HashSet::new();

            for sequence in supernet_sequences {
                sequence.as_bytes().windows(3).for_each(|window| {
                    if window[0] != window[1] && window[0] == window[2] {
                        babs.insert([window[1], window[0], window[1]]);
                    }
                });
            }

            hypernet_sequence.iter().any(|sequence| {
                sequence.as_bytes().windows(3).any(|window| {
                    window[0] != window[1] && window[0] == window[2] && babs.contains(window)
                })
            })
        })
        .count();

    dbg!(count);
}
