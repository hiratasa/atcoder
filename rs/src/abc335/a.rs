use std::iter::once;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let n = s.len();
    println!("{}", s.into_iter().take(n - 1).chain(once('4')).join(""));
}
