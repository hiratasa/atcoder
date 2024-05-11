use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let ans = t
        .into_iter()
        .enumerate()
        .scan(0, |i, (j, c)| {
            if *i >= s.len() {
                None
            } else if s[*i] == c {
                *i += 1;
                Some(Some(j))
            } else {
                Some(None)
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    println!("{}", ans.iter().map(|i| i + 1).join(" "))
}
