use md5::{Digest, Md5};

fn main() {
    let input = include_str!(r"..\..\input\day5.txt").trim();
    let mut i = 0;

    let mut password = [' '; 8];
    let mut pass_chars = 0;

    loop {
        let digest = Md5::digest(format!("{input}{i}").as_str());
        let digest = digest.as_slice();

        if matches!(digest[0..3], [0, 0, 0..=0x0f]) {
            password[pass_chars] = char::from_digit(digest[2].into(), 16).unwrap();
            pass_chars += 1;

            if pass_chars == 8 {
                break;
            }
        }

        i += 1;
    }

    let password = password.iter().collect::<String>();

    dbg!(password);
}
