fn main() {
    let input = include_str!(r"..\..\input\day9.txt").trim();

    let len = decompress(input);

    dbg!(len);
}

fn decompress(s: &str) -> usize {
    let mut input = s.chars().peekable();

    let mut decompressed_len = 0;

    while input.peek().is_some() {
        let ch = input.next().unwrap();

        if ch == '(' {
            let mut length = String::new();

            while input.peek().copied() != Some('x') {
                length.push(input.next().unwrap());
            }

            input.next();

            let mut times = String::new();

            while input.peek().copied() != Some(')') {
                times.push(input.next().unwrap());
            }

            input.next();

            let length = length.parse::<usize>().unwrap();
            let times = times.parse::<usize>().unwrap();

            let substr_len = decompress(&input.by_ref().take(length).collect::<String>());

            for _ in 0..times {
                decompressed_len += substr_len;
            }
        } else {
            decompressed_len += 1;
        }
    }

    decompressed_len
}
