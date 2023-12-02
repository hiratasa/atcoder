use std::iter::once;

use itertools::Itertools;
use itertools_num::ItertoolsNum;
use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let values = a
        .iter()
        .copied()
        .sorted()
        .group_by(|&x| x)
        .into_iter()
        .map(|(x, it)| (x, it.count()))
        .collect::<Vec<_>>();
    let idxs = values
        .iter()
        .copied()
        .enumerate()
        .map(|(i, (x, _))| (x, i))
        .collect::<FxHashMap<_, _>>();
    let m = values.len();

    let sums = once(0)
        .chain(values.iter().copied().map(|(x, y)| x * y))
        .cumsum::<usize>()
        .collect::<Vec<_>>();

    println!(
        "{}",
        (0..n)
            .map(|i| {
                let x = a[i];
                let idx = idxs[&x];

                sums[m] - sums[idx] - x * values[idx].1
            })
            .join(" ")
    );
}
