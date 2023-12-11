use itertools::Itertools;
use regex::Regex;

fn main() {
    let mut screen = [[false; 50]; 6];

    let rect_pattern = Regex::new(r"^rect (\d+)x(\d)$").unwrap();
    let rotate_row_pattern = Regex::new(r"^rotate row y=(\d) by (\d+)$").unwrap();
    let rotate_col_pattern = Regex::new(r"^rotate column x=(\d+) by (\d+)$").unwrap();

    for line in include_str!(r"..\..\input\day8.txt").lines() {
        if let Some(captures) = rect_pattern.captures(line) {
            let width = captures[1].parse::<usize>().unwrap();
            let height = captures[2].parse::<usize>().unwrap();

            for (x, y) in (0..height).cartesian_product(0..width) {
                screen[x][y] = true;
            }
        } else if let Some(captures) = rotate_row_pattern.captures(line) {
            let row = captures[1].parse::<usize>().unwrap();
            let shift = captures[2].parse::<usize>().unwrap();

            screen[row].rotate_right(shift);
        } else if let Some(captures) = rotate_col_pattern.captures(line) {
            let col = captures[1].parse::<usize>().unwrap();
            let shift = captures[2].parse::<usize>().unwrap();

            let mut extracted_col = screen.map(|row| row[col]);
            extracted_col.rotate_right(shift);

            for i in 0..6 {
                screen[i][col] = extracted_col[i];
            }
        }
    }

    let lit = screen.iter().flatten().filter(|cell| **cell).count();

    dbg!(lit);
}
