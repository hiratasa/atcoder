use std::iter::once;

use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let a0 = -once(0)
        .chain(a.iter().copied())
        .cumsum::<i64>()
        .min()
        .unwrap();
    let ans = once(a0).chain(a).sum::<i64>();

    println!("{ans}");
}
