use itertools::Itertools;
use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        h: usize, w: usize, m: usize,
        tax: [(usize, usize, usize); m],
    };

    let (mut ans, hh, ww) = tax.into_iter().rev().unique_by(|&(t, a, _)| (t, a)).fold(
        (FxHashMap::default(), h, w),
        |(mut ans, hh, ww), (t, _a, x)| {
            if t == 1 {
                *ans.entry(x).or_insert(0) += ww;

                (ans, hh - 1, ww)
            } else {
                *ans.entry(x).or_insert(0) += hh;

                (ans, hh, ww - 1)
            }
        },
    );

    *ans.entry(0).or_insert(0) += hh * ww;

    ans.retain(|_, k| *k > 0);

    println!("{}", ans.len());

    for (color, k) in ans.into_iter().sorted() {
        println!("{color} {k}");
    }
}
