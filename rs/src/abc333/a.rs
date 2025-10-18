use itertools::{Itertools, repeat_n};
use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    println!("{}", repeat_n(n, n).join(""));
}
