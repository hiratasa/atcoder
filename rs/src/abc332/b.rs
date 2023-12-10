use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        k: usize, g: usize, m: usize,
    };

    let ans = (0..k).fold((0, 0), |(glass, mag), _| match (glass, mag) {
        (gg, _) if gg == g => (0, mag),
        (_, 0) => (glass, m),
        (_, _) => {
            let r = min(mag, g - glass);

            (glass + r, mag - r)
        }
    });

    println!("{} {}", ans.0, ans.1);
}
