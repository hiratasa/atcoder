use std::cmp::min;

use itertools::izip;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let left = a
        .iter()
        .scan(1, |l, &x| {
            if *l <= x {
                *l += 1;
                Some(*l - 1)
            } else {
                *l = x + 1;
                Some(x)
            }
        })
        .collect::<Vec<_>>();
    let mut right = a
        .iter()
        .rev()
        .scan(1, |l, &x| {
            if *l <= x {
                *l += 1;
                Some(*l - 1)
            } else {
                *l = x + 1;
                Some(x)
            }
        })
        .collect::<Vec<_>>();
    right.reverse();

    let ans = izip!(left, right).map(|(l, r)| min(l, r)).max().unwrap();

    println!("{ans}");
}
