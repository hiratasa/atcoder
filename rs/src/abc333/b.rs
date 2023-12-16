use std::cmp::min;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let len_s = min(
        (s[1] as usize + 5 - s[0] as usize) % 5,
        (s[0] as usize + 5 - s[1] as usize) % 5,
    );
    let len_t = min(
        (t[1] as usize + 5 - t[0] as usize) % 5,
        (t[0] as usize + 5 - t[1] as usize) % 5,
    );

    if len_s == len_t {
        println!("Yes");
    } else {
        println!("No");
    }
}
