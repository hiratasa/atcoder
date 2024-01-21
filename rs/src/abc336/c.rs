use std::iter::successors;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    if n == 1 {
        println!("0");
        return;
    }

    let ans = successors(Some(n - 1), |&m| Some(m / 5))
        .take_while(|&m| m > 0)
        .map(|m| m % 5 * 2)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>();

    println!("{}", ans.iter().join(""));
}
