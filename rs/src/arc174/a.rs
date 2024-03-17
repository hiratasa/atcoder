use std::cmp::*;

use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {
        n: usize, c: i64,
        a: [i64; n],
    };

    let sums = a.iter().copied().cumsum::<i64>().collect::<Vec<_>>();

    let ma = sums
        .iter()
        .copied()
        .scan(0, |y, x| {
            *y = min(x, *y);

            Some(x - *y)
        })
        .max()
        .unwrap();
    let mi = sums
        .iter()
        .copied()
        .scan(0, |y, x| {
            *y = max(x, *y);

            Some(x - *y)
        })
        .min()
        .unwrap();

    let ans = sums[n - 1] + 0i64.max((c - 1) * ma).max((c - 1) * mi);

    println!("{ans}");
}
