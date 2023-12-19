use std::{cmp::Ordering, iter::once};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        mut a: [usize; n],
    };

    a.sort();

    let freq = a
        .iter()
        .copied()
        .group_by(|&x| x)
        .into_iter()
        .map(|(x, it)| (x, it.count()))
        .collect::<Vec<_>>();

    let dmax = freq.iter().copied().map(|(_, d)| d).max().unwrap();

    let ans = if dmax == 1 {
        let z = a
            .iter()
            .copied()
            .chain(once(k + a[0]))
            .tuple_windows()
            .map(|(x, y)| y - x)
            .max()
            .unwrap();

        k * (n - 2) + z
    } else {
        let dmaxs = freq
            .iter()
            .copied()
            .filter(|&(_, d)| d == dmax)
            .map(|(x, _)| x)
            .collect::<Vec<_>>();
        let z0 = dmaxs[0];
        let z1 = dmaxs[dmaxs.len() - 1];

        a.iter()
            .copied()
            .map(|x| {
                if z1 <= x {
                    k * (n - 1 - (dmax - 1)) + z0 - x
                } else {
                    let idx = dmaxs
                        .binary_search_by(|&z| z.cmp(&x).then(Ordering::Less))
                        .unwrap_err();
                    let z = dmaxs[idx];
                    k * (n - dmax - 1) + z - x
                }
            })
            .max()
            .unwrap()
    };

    println!("{ans}");
}
