use std::iter::once;

use itertools::{Itertools, izip};
use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; k],
    };

    let ans = if k % 2 == 0 {
        a.iter()
            .copied()
            .tuples()
            .map(|(x, y)| y - x)
            .sum::<usize>()
    } else {
        let from_lefts = once(0)
            .chain(a.iter().copied().tuples().map(|(x, y)| y - x))
            .cumsum::<usize>()
            .collect::<Vec<_>>();
        let from_right = once(0)
            .chain(a.iter().copied().rev().tuples().map(|(x, y)| x - y))
            .cumsum::<usize>()
            .collect::<Vec<_>>();

        izip!(from_lefts, from_right.into_iter().rev())
            .map(|(x, y)| x + y)
            .min()
            .unwrap()
    };

    println!("{ans}");
}
