use std::cmp::{max, min};

use proconio::input;

fn main() {
    input! {
        n: usize, x: usize, y: usize, z: usize,
    };

    if min(x, y) <= z && z <= max(x, y) {
        println!("Yes");
    } else {
        println!("No");
    }
}
