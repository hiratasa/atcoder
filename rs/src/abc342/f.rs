use std::{cmp::min, iter::once};

use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {
        n: usize, l: usize, d: usize,
    };

    let mut init = vec![0.0; l + d + 1];
    init[0] = 1.0;
    let t = (0..l + d).fold(init, |mut t, i| {
        if i < l {
            let p = t[i] / d as f64;
            t[i + 1] += p;
            t[i + d + 1] -= p;
        }

        if i > 0 {
            t[i + 1] += t[i];
        }

        if i < l {
            t[i] = 0.0;
        }

        t
    });

    let p_y_burst = (min(n + 1, l + d + 1)..l + d + 1)
        .map(|i| t[i])
        .sum::<f64>();
    let cumsum = once(0.0)
        .chain(t.iter().copied())
        .cumsum::<f64>()
        .collect::<Vec<_>>();

    let p_win = (0..=n).rev().fold(vec![0.0; n + d + 2], |mut p_win, i| {
        p_win[i + 1] += p_win[i + 2];

        let p = (p_win[i + 1] - p_win[i + d + 1]) / d as f64;
        let q = if i < cumsum.len() {
            p_y_burst + cumsum[i]
        } else {
            p_y_burst + cumsum[cumsum.len() - 1]
        };

        p_win[i] = f64::max(p, q);

        p_win
    });
    let ans = p_win[0];

    println!("{ans}");
}
