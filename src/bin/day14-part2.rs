use std::collections::HashMap;

use md5::{Digest, Md5};

fn main() {
    let salt = include_str!(r"..\..\input\day14.txt").trim();

    let mut index = 0;
    let mut found_keys = 0;

    let mut cache: HashMap<String, String> = HashMap::new();

    loop {
        let plain = format!("{salt}{index}");
        let digest = cache.get(&plain).cloned().map_or_else(
            || {
                let digest = Md5::digest(plain.as_str());
                let mut digest = hex::encode(digest.as_slice());

                for _ in 0..2016 {
                    digest = hex::encode(Md5::digest(digest).as_slice());
                }

                cache.insert(plain, digest.clone());

                digest
            },
            |hash| hash,
        );

        let digest = digest.as_bytes();

        if let Some(triplet) = digest
            .windows(3)
            .find(|&window| window[0] == window[1] && window[0] == window[2])
        {
            let mut found = false;

            for i in index + 1..=index + 1000 {
                let input = format!("{salt}{i}");
                let digest = cache.get(&input).cloned().map_or_else(
                    || {
                        let digest = Md5::digest(input.as_str());
                        let mut digest = hex::encode(digest.as_slice());

                        for _ in 0..2016 {
                            digest = hex::encode(Md5::digest(digest).as_slice());
                        }

                        cache.insert(input, digest.clone());

                        digest
                    },
                    |hash| hash,
                );

                let digest = digest.as_bytes();

                if let Some(pentuplet) = digest.windows(5).find(|&window| {
                    window[0] == window[1]
                        && window[1] == window[2]
                        && window[2] == window[3]
                        && window[3] == window[4]
                }) {
                    if pentuplet[0] == triplet[0] {
                        found = true;
                        break;
                    }
                }
            }

            if found {
                found_keys += 1;
            }
        }

        if found_keys == 64 {
            dbg!(index);
            break;
        }

        index += 1;
    }
}
