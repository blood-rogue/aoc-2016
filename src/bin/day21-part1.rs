use itertools::Itertools;
use regex::Regex;

const SWAP_POS_RE: &str = r"^swap position (\d) with position (\d)$";
const SWAP_CHAR_RE: &str = r"^swap letter ([a-z]) with letter ([a-z])$";
const ROTATE_STEPS_RE: &str = r"^rotate (left|right) (\d) steps?$";
const ROTATE_CHAR_RE: &str = r"^rotate based on position of letter ([a-z])$";
const REVERSE_RE: &str = r"^reverse positions (\d) through (\d)$";
const MOVE_RE: &str = r"^move position (\d) to position (\d)$";

fn main() {
    let swap_pos_pattern = Regex::new(SWAP_POS_RE).unwrap();
    let swap_char_pattern = Regex::new(SWAP_CHAR_RE).unwrap();
    let rotate_steps_pattern = Regex::new(ROTATE_STEPS_RE).unwrap();
    let rotate_char_pattern = Regex::new(ROTATE_CHAR_RE).unwrap();
    let reverse_pattern = Regex::new(REVERSE_RE).unwrap();
    let move_pattern = Regex::new(MOVE_RE).unwrap();

    let mut password = "abcdefgh".chars().collect_vec();

    for line in include_str!(r"..\..\input\day21.txt").lines() {
        if let Some(captures) = swap_pos_pattern.captures(line) {
            let from = captures[1].parse::<usize>().unwrap();
            let to = captures[2].parse::<usize>().unwrap();

            password.swap(from, to);
        }

        if let Some(captures) = swap_char_pattern.captures(line) {
            let from = captures[1].parse::<char>().unwrap();
            let to = captures[2].parse::<char>().unwrap();

            password = password
                .iter()
                .map(|&ch| {
                    if ch == from {
                        to
                    } else if ch == to {
                        from
                    } else {
                        ch
                    }
                })
                .collect();
        }

        if let Some(captures) = rotate_steps_pattern.captures(line) {
            let direction = &captures[1];
            let steps = captures[2].parse::<usize>().unwrap();

            if direction == "left" {
                password.rotate_left(steps);
            } else {
                password.rotate_right(steps);
            }
        }

        if let Some(captures) = rotate_char_pattern.captures(line) {
            let ch = captures[1].parse::<char>().unwrap();

            let pos = password.iter().position(|&c| c == ch).unwrap();
            let steps = if pos >= 4 { pos + 1 } else { pos };

            password.rotate_right(1);
            password.rotate_right(steps);
        }

        if let Some(captures) = reverse_pattern.captures(line) {
            let mut low = captures[1].parse::<usize>().unwrap();
            let mut high = captures[2].parse::<usize>().unwrap();

            while high > low {
                password.swap(low, high);

                low += 1;
                high -= 1;
            }
        }

        if let Some(captures) = move_pattern.captures(line) {
            let from = captures[1].parse::<usize>().unwrap();
            let to = captures[2].parse::<usize>().unwrap();

            let ch = password.remove(from);
            password.insert(to, ch);
        }
    }

    let password = password.iter().collect::<String>();

    dbg!(password);
}
