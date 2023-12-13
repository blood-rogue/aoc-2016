use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Node {
    index: (usize, usize),
    size: usize,
    used: usize,
    avail: usize,
}

const NODE_RE: &str = r"^/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T";

fn main() {
    let node_pattern = Regex::new(NODE_RE).unwrap();
    let mut count = 0;

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
            }
        });

    for (node1, node2) in nodes.permutations(2).map(|perm| (perm[0], perm[1])) {
        if node1.index != node2.index && node1.used != 0 && node1.used <= node2.avail {
            count += 1;
        }
    }

    dbg!(count);
}
