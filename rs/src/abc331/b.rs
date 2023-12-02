use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        n: usize, s: usize, m: usize, l: usize,
    };

    let ans = iproduct!(0..20, 0..15, 0..10)
        .filter(|&(i, j, k)| 6 * i + 8 * j + 12 * k >= n)
        .map(|(i, j, k)| s * i + m * j + l * k)
        .min()
        .unwrap();

    println!("{ans}");
}
