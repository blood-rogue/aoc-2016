use std::collections::HashMap;

fn main() {
    let fav_num = include_str!(r"..\..\input\day13.txt")
        .trim()
        .parse::<isize>()
        .unwrap();

    let destination = (31, 39);

    let mut frontier = vec![(1, 1, 0)];
    let mut explored = HashMap::new();

    let is_space = |x: isize, y: isize| -> bool {
        (x * x + 3 * x + 2 * x * y + y + y * y + fav_num).count_ones() % 2 == 0 && x > 0 && y > 0
    };

    while let Some((x, y, dist)) = frontier.pop() {
        explored.insert((x, y), dist);

        for neighbour in [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .into_iter()
            .map(|(i, j)| (x + i, y + j, dist + 1))
            .filter(|&(x, y, _)| is_space(x, y))
        {
            if !explored.contains_key(&(neighbour.0, neighbour.1))
                || explored[&(neighbour.0, neighbour.1)] > neighbour.2
            {
                frontier.push(neighbour);
            }
        }
    }

    let steps = explored[&destination];

    dbg!(steps);
}
