use std::{cmp::min, ops::Deref};

use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: Chars,
        n: usize,
        s: [[Chars]; n],
    };

    let m = t.len();

    let mut init = vec![usize::MAX; m + 1];
    init[0] = 0;
    let dp = s.into_iter().fold(init, |dp, ss| {
        iproduct!((0..m).rev(), ss)
            .filter(|(i, x)| &t[*i..min(m, i + x.len())] == x.deref())
            .fold(dp, |mut dp, (i, x)| {
                dp[i + x.len()] = min(dp[i + x.len()], dp[i].saturating_add(1));
                dp
            })
    });

    if dp[m] == usize::MAX {
        println!("-1");
    } else {
        println!("{}", dp[m]);
    }
}
