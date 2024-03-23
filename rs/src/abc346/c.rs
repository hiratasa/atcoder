use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n],
    };

    let s = a
        .iter()
        .copied()
        .filter(|&x| x <= k)
        .unique()
        .sum::<usize>();

    let ans = k * (k + 1) / 2 - s;

    println!("{}", ans);
}
