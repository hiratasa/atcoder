use std::mem::replace;

use itertools::{iterate, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let t = a
        .into_iter()
        .map(|x| {
            (2..)
                .scan(x, |y, i| {
                    if *y == 1 {
                        None
                    } else if i * i > *y {
                        Some((replace(y, 1), 1))
                    } else {
                        let d;
                        (d, *y) = iterate(*y, |&z| z / i)
                            .enumerate()
                            .find(|&(_, z)| z % i > 0)
                            .unwrap();

                        Some((i, d))
                    }
                })
                .filter(|&(_, d)| d % 2 > 0)
                .map(|(p, _)| p)
                .collect::<Vec<_>>()
        })
        .counts();

    let m = t.get(&vec![0]).copied().unwrap_or_default();
    let ans = m * (n - m) + t.values().map(|&c| c * (c - 1) / 2).sum::<usize>();

    println!("{ans}");
}
