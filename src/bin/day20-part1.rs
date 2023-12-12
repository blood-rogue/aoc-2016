use itertools::Itertools;

fn main() {
    let ranges = include_str!(r"..\..\input\day20.txt")
        .lines()
        .map(|line| {
            line.split_once('-')
                .map(|(s, e)| (s.parse::<u32>().unwrap(), e.parse::<u32>().unwrap()))
                .expect(line)
        })
        .sorted()
        .collect_vec();

    let mut min = 0;

    for (start, end) in ranges {
        if (start..=end).contains(&min) {
            min = end + 1
        }
    }

    dbg!(min);
}
