use itertools::Itertools;

fn main() {
    let mut state = include_str!(r"..\..\input\day16.txt")
        .trim()
        .chars()
        .map(|ch| ch == '1')
        .collect_vec();

    while state.len() < 272 {
        state = [
            state.clone(),
            vec![false],
            state.iter().rev().map(|b| !b).collect_vec(),
        ]
        .concat();
    }

    state = state[..272].to_vec();
    let mut parity = state;

    while parity.len() % 2 == 0 {
        parity = parity
            .chunks_exact(2)
            .map(|chunk| !(chunk[0] ^ chunk[1]))
            .collect();
    }

    let checksum = parity
        .iter()
        .map(|b| if *b { '1' } else { '0' })
        .collect::<String>();

    dbg!(checksum);
}
