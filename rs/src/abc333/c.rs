use itertools::{iterate, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let ans = iterate(1usize, |&x| x * 10 + 1)
        .take(15)
        .combinations_with_replacement(3)
        .map(|v| v.into_iter().sum::<usize>())
        .sorted()
        .dedup()
        .nth(n - 1)
        .unwrap();

    println!("{ans}");
}
