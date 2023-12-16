use itertools::{repeat_n, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    println!("{}", repeat_n(n, n).join(""));
}
