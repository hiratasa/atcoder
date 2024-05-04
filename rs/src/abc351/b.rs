use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
        b: [Chars; n],
    };

    let ans = iproduct!(0..n, 0..n)
        .find(|&(i, j)| a[i][j] != b[i][j])
        .unwrap();

    println!("{} {}", ans.0 + 1, ans.1 + 1);
}
