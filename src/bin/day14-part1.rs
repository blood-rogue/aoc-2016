use md5::{Digest, Md5};

fn main() {
    let salt = include_str!(r"..\..\input\day14.txt").trim();

    let mut index = 0;
    let mut found_keys = 0;

    loop {
        let digest = Md5::digest(format!("{salt}{index}").as_str());
        let digest = hex::encode(digest.as_slice());
        let digest = digest.as_bytes();

        if let Some(triplet) = digest
            .windows(3)
            .find(|&window| window[0] == window[1] && window[0] == window[2])
        {
            let mut found = false;

            for i in index + 1..=index + 1000 {
                let digest = Md5::digest(format!("{salt}{i}").as_str());
                let digest = hex::encode(digest.as_slice());
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
