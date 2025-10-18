use itertools::{Itertools, iproduct};
use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    iproduct!(0..=n, 0..=n, 0..=n)
        .filter(|&(i, j, k)| i + j + k <= n)
        .for_each(|(i, j, k)| {
            println!("{i} {j} {k}");
        });
}
