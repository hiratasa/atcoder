use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut ans = s
        .into_iter()
        .rev()
        .take_while(|&c| c != '.')
        .collect::<Vec<_>>();
    ans.reverse();

    println!("{}", ans.iter().join(""));
}
