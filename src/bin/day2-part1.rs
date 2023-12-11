fn main() {
    let numpad = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

    let mut row = 1;
    let mut col = 1;

    let password = include_str!(r"..\..\input\day2.txt")
        .trim()
        .lines()
        .map(|line| {
            let instructions = line.as_bytes();

            for instruction in instructions {
                match instruction {
                    b'D' if row < 2 => row += 1,
                    b'U' if row > 0 => row -= 1,
                    b'R' if col < 2 => col += 1,
                    b'L' if col > 0 => col -= 1,
                    _ => {}
                }
            }

            numpad[row][col]
        })
        .collect::<String>();

    dbg!(password);
}
