use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
struct Node {
    index: (usize, usize),
    size: usize,
    used: usize,
    avail: usize,
    used_percent: usize,
}

fn main() {
    // generate_grid();

    const DISTANCE: usize =
        distance((16, 23), (5, 12)) + distance((5, 12), (37, 0)) + distance((36, 0), (0, 0)) * 5;

    dbg!(DISTANCE);
}

const fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

#[allow(dead_code)]
fn generate_grid() {
    let node_pattern =
        Regex::new(r"^/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%").unwrap();

    let nodes = include_str!(r"..\..\input\day22.txt")
        .lines()
        .skip(2)
        .map(|line| {
            let captures = node_pattern.captures(line).unwrap();
            Node {
                index: (captures[1].parse().unwrap(), captures[2].parse().unwrap()),
                size: captures[3].parse().unwrap(),
                used: captures[4].parse().unwrap(),
                avail: captures[5].parse().unwrap(),
                used_percent: captures[6].parse().unwrap(),
            }
        })
        .collect_vec();

    let mut grid = [[Node::default(); 38]; 26];
    let mut rows = Vec::with_capacity(28);

    rows.push(String::with_capacity(41));
    rows.push(String::with_capacity(41));

    rows.get_mut(0).unwrap().push_str("   ");
    rows.get_mut(1).unwrap().push_str("   ");

    for i in 0..38 {
        rows.get_mut(0).unwrap().push(if i < 10 {
            ' '
        } else {
            char::from_digit(i / 10, 10).unwrap()
        });

        rows.get_mut(1)
            .unwrap()
            .push(char::from_digit(i % 10, 10).unwrap());
    }

    for node in nodes {
        let (x, y) = node.index;
        grid[y][x] = node;
    }

    for (i, row) in grid.iter().enumerate() {
        let mut row_str = String::with_capacity(41);

        if i < 10 {
            row_str.push(' ');
            row_str.push(char::from_digit(u32::try_from(i).unwrap(), 10).unwrap());
        } else {
            row_str.push(char::from_digit(u32::try_from(i).unwrap() / 10, 10).unwrap());
            row_str.push(char::from_digit(u32::try_from(i).unwrap() % 10, 10).unwrap());
        }

        row_str.push(' ');

        for (j, node) in row.iter().enumerate() {
            let marker = if i == 0 && j == 0 {
                'X'
            } else if j == 37 && i == 0 {
                'G'
            } else if node.used == 0 {
                'O'
            } else if node.size < 100 {
                '.'
            } else {
                '#'
            };

            row_str.push(marker);
        }

        rows.push(row_str);
    }

    std::fs::write("output/grid.txt", rows.join("\n")).unwrap();
}
