fn main() {
    let n = include_str!(r"..\..\input\day19.txt")
        .trim()
        .parse::<isize>()
        .unwrap();

    let mut i = 1;

    while i * 3 < n {
        i *= 3
    }

    let winner = n - i;

    dbg!(winner);
}
