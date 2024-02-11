use std::iter::successors;

use itertools::iproduct;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        p: usize, b: usize, n: usize, m: usize,
        lr: [(Usize1, usize); m],
    };

    // if p >= n {
    //     println!("Yes");
    //     return;
    // }

    let mut connected = vec![vec![false; n + 1]; n + 1];
    for (l, r) in lr {
        connected[l][r] = true;
        connected[r][l] = true;
    }

    let table = (0usize..1 << (n + 1))
        .map(|s| {
            iproduct!(
                (0..(n + 1)).filter(|&i| s & (1 << i) > 0),
                (0..(n + 1)).filter(|&i| s & (1 << i) > 0)
            )
            .all(|(i, j)| !connected[i][j])
        })
        .collect::<Vec<_>>();

    let dp = (1usize..1 << (n + 1)).fold(vec![0; 1 << (n + 1)], |mut dp, s| {
        dp[s] = successors(Some(s), |&t| t.checked_sub(1).map(|t| t & s))
            .skip(1)
            .filter(|&t| table[s ^ t])
            .map(|t| dp[t] + 1)
            .min()
            .unwrap();

        dp
    });

    // eprintln!("{dp:?}");

    if dp[(1 << (n + 1)) - 1] <= p {
        println!("Yes");
    } else {
        println!("No");
    }
}
