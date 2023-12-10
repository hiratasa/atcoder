use std::{cmp::min, iter::successors};

use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        n: usize, d: usize,
        w: [usize; n],
    };

    let sums = (0..1 << n)
        .map(|s| {
            (0..n)
                .filter(|&i| s & (1 << i) > 0)
                .map(|i| w[i])
                .sum::<usize>()
        })
        .collect::<Vec<_>>();

    let dp =
        iproduct!(0..d, 0..1 << n).fold(vec![vec![usize::MAX; 1 << n]; d + 1], |mut dp, (i, s)| {
            if s == 0 {
                dp[i][s] = 0;
            }

            let not_s = ((1 << n) - 1) ^ s;
            successors(Some(not_s), |&t| t.checked_sub(1).map(|t| t & not_s)).for_each(|t| {
                dp[i + 1][s ^ t] = min(dp[i + 1][s ^ t], dp[i][s].saturating_add(sums[t].pow(2)));
            });

            dp
        });

    let sum2 = dp[d][(1 << n) - 1];

    let ans = (sum2 as u128 * d as u128 - sums[(1 << n) - 1].pow(2) as u128) as f64 as f64
        / (d as f64).powi(2);

    println!("{ans}");
}
