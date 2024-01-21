use std::iter::repeat;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    println!("L{}ng", repeat('o').take(n).join(""));
}
