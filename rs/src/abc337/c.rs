use std::iter::successors;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let tail = a
        .iter()
        .copied()
        .fold(vec![false; n], |mut seen, idx| {
            if idx > 0 {
                seen[(idx - 1) as usize] = true;
            }
            seen
        })
        .into_iter()
        .position(|x| !x)
        .unwrap();

    let mut ans = successors(Some(tail), |&idx| (a[idx] - 1).try_into().ok()).collect::<Vec<_>>();
    ans.reverse();

    println!("{}", ans.iter().map(|idx| idx + 1).join(" "));
}
