use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Node {
    Wall,
    Space,
    Point(u8),
}

impl Node {
    const fn is_open(self) -> bool {
        !matches!(self, Self::Wall)
    }

    const fn is_point(self) -> bool {
        matches!(self, Self::Point(_))
    }
}

impl Node {
    fn parse(s: &str) -> Self {
        match s {
            "#" => Self::Wall,
            "." => Self::Space,
            _ => Self::Point(s.parse().unwrap()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Grid {
    points: usize,
    data: Vec<Vec<Node>>,
}

impl Grid {
    fn new(g: Vec<&str>) -> Self {
        let mut points = 0;
        let data = g
            .into_iter()
            .map(|s| {
                let l = s.len();
                s.split("")
                    .skip(1)
                    .take(l)
                    .map(|c| {
                        let c = Node::parse(c);
                        if c.is_point() {
                            points += 1;
                        }

                        c
                    })
                    .collect()
            })
            .collect();

        Self { points, data }
    }

    fn get(&self, x: usize, y: usize) -> Node {
        self.data[y][x]
    }

    fn get_neighbors(&self, p: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = p;
        let mut neighbors = vec![(x + 1, y), (x, y + 1)];

        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }

        neighbors
            .into_iter()
            .filter(|&(x, y)| self.get(x, y).is_open())
            .collect()
    }

    fn point_pos(&self, p: usize) -> Option<(usize, usize)> {
        let p = u8::try_from(p).unwrap();

        for y in 0..self.data.len() {
            for x in 0..self.data[0].len() {
                match self.get(x, y) {
                    Node::Point(q) if p == q => return Some((x, y)),
                    _ => {}
                }
            }
        }

        None
    }
}

pub fn main() {
    let input = include_str!(r"..\..\input\day24.txt").lines().collect_vec();

    let grid = Grid::new(input);

    let paths = (0..grid.points)
        .tuple_combinations::<(usize, usize)>()
        .collect_vec();
    let mut path_distances = vec![vec![0; grid.points]; grid.points];

    for (x, y) in paths {
        let initial = grid.point_pos(x).unwrap();
        let goal = grid.point_pos(y).unwrap();

        let mut distances = HashMap::new();
        let mut visited = HashSet::new();
        let mut to_visit = Vec::new();

        distances.insert(initial, 0usize);
        let mut current = initial;

        while !distances.contains_key(&goal) {
            let distance = *distances.get(&current).unwrap();
            visited.insert(current);

            for loc in grid.get_neighbors(current) {
                if visited.contains(&loc) {
                    continue;
                }

                let dist = *distances.entry(loc).or_insert(distance + 1);
                if dist > distance + 1 {
                    distances.insert(loc, distance + 1);
                }

                to_visit.push((*distances.get(&loc).unwrap(), loc));
            }

            to_visit.sort_by_key(|&(d, (x, y))| {
                let x = x as isize - goal.0 as isize;
                let y = y as isize - goal.1 as isize;

                -(d as isize + x.abs() + y.abs())
            });

            current = to_visit.pop().unwrap().1;
        }

        path_distances[x][y] = *distances.get(&goal).unwrap();
        path_distances[y][x] = *distances.get(&goal).unwrap();
    }

    let mut distance = 100_000;

    for ordering in (1..grid.points).permutations(grid.points - 1) {
        let mut dist = path_distances[0][ordering[0]];

        for (x, y) in ordering.iter().copied().tuple_windows() {
            dist += path_distances[x][y];
        }

        dist += path_distances[ordering[ordering.len() - 1]][0];

        if distance > dist {
            distance = dist;
        }
    }

    dbg!(distance);
}
