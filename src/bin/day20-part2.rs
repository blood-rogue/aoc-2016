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

    let mut count = 0;
    let mut index = 0;
    let mut ip = 0;

    while ip < u32::MAX {
        let (start, end) = ranges[index];

        if ip >= start {
            if ip <= end {
                if end < u32::MAX {
                    ip = end + 1;
                } else {
                    ip = u32::MAX;
                }
                continue;
            }

            index += 1;
        } else {
            count += 1;
            ip += 1;
        }
    }

    dbg!(count);
}
