fn main() {
    let n = include_str!(r"..\..\input\day19.txt")
        .trim()
        .parse::<usize>()
        .unwrap();

    let winner = (n ^ 1 << (usize::BITS - 1 - n.leading_zeros())) << 1 | 1;

    dbg!(winner);
}
