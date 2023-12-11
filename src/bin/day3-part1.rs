fn main() {
    let count = include_str!(r"..\..\input\day3.txt")
        .lines()
        .map(|line| {
            (
                line[2..5].trim().parse::<usize>().unwrap(),
                line[7..10].trim().parse::<usize>().unwrap(),
                line[12..].trim().parse::<usize>().unwrap(),
            )
        })
        .filter(|&(a, b, c)| a + b > c && a + c > b && b + c > a)
        .count();

    dbg!(count);
}
