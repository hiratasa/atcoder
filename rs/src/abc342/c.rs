use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        query: [(char, char)],
    };

    let mapping = query
        .into_iter()
        .fold((0..26).collect::<Vec<_>>(), |mut mapping, (c, d)| {
            let c = c as u8 - b'a';
            for i in 0..26 {
                if mapping[i] == c {
                    mapping[i] = d as u8 - b'a';
                }
            }
            mapping
        });

    println!(
        "{}",
        s.into_iter()
            .map(|c| { (mapping[c as usize - 'a' as usize] + b'a') as char })
            .join("")
    );
}
