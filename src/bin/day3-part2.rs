use itertools::Itertools;

fn main() {
    let count = include_str!(r"..\..\input\day3.txt")
        .lines()
        .chunks(3)
        .into_iter()
        .flat_map(|mut chunk| {
            let mut parsed_lines = [[0; 3]; 3];

            for i in 0..3 {
                let line = chunk.next().unwrap();

                parsed_lines[0][i] = line[2..5].trim().parse::<usize>().unwrap();
                parsed_lines[1][i] = line[7..10].trim().parse::<usize>().unwrap();
                parsed_lines[2][i] = line[12..].trim().parse::<usize>().unwrap();
            }

            parsed_lines
        })
        .filter(|&[a, b, c]| a + b > c && a + c > b && b + c > a)
        .count();

    dbg!(count);
}
