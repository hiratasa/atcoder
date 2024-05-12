use std::iter::once;

use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n],
    };

    let ans = a
        .into_iter()
        .chain(once(usize::MAX))
        .scan(k, |c, x| {
            if x == usize::MAX {
                Some(1)
            } else if *c < x {
                *c = k - x;
                Some(1)
            } else {
                *c -= x;
                Some(0)
            }
        })
        .sum::<usize>();

    println!("{ans}");
}
