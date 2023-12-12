use std::collections::{HashMap, VecDeque};

use md5::{Digest, Md5};

fn main() {
    let input = include_str!(r"..\..\input\day17.txt").trim().to_string();

    let moves = HashMap::from([
        (
            'U',
            Box::new(|x, y| (x, y - 1isize)) as Box<dyn Fn(isize, isize) -> (isize, isize)>,
        ),
        ('D', Box::new(|x, y| (x, y + 1))),
        ('L', Box::new(|x, y| (x - 1, y))),
        ('R', Box::new(|x, y| (x + 1, y))),
    ]);

    let start = (0, 0);
    let goal = (3, 3);

    let mut paths = Vec::new();

    let mut queue = VecDeque::from([(start, vec![start], Vec::<char>::new())]);

    while !queue.is_empty() {
        let ((x, y), path, dirs) = queue.pop_front().unwrap();

        for (dir, _) in ['U', 'D', 'L', 'R']
            .iter()
            .zip(doors(input.clone(), &dirs))
            .filter(|&(_, door)| door)
        {
            let next = moves[dir](x, y);

            let (nx, ny) = next;

            if !((0..4).contains(&nx) && (0..4).contains(&ny)) {
                continue;
            }

            if next == goal {
                paths.push([dirs.clone(), vec![*dir]].concat());
            } else {
                queue.push_back((
                    next,
                    [path.clone(), vec![next]].concat(),
                    [dirs.clone(), vec![*dir]].concat(),
                ));
            }
        }
    }

    let len = paths.last().unwrap().len();

    dbg!(len);
}

fn doors(mut input: String, path: &[char]) -> Vec<bool> {
    input.extend(path.iter());
    let s = input;

    let digest = Md5::digest(s);
    hex::encode(digest.as_slice())
        .get(..4)
        .unwrap()
        .chars()
        .map(|ch| ch.to_digit(16).unwrap() > 10)
        .collect()
}
