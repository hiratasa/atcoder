use std::iter::once;

use itertools::{Itertools, izip};
use proconio::input;

fn main() {
    input! {
        a: [usize; 6],
        n: usize,
        x: [usize; n],
    };

    const COINS: [usize; 6] = [1, 5, 10, 50, 100, 500];

    let nums = x
        .iter()
        .map(|&xx| {
            COINS
                .into_iter()
                .rev()
                .scan(xx, |xx, c| {
                    let m = *xx / c;
                    *xx %= c;

                    Some(m)
                })
                .collect::<Vec<_>>()
        })
        .fold([0; 6], |mut nums, nums1| {
            izip!(nums.iter_mut(), nums1.into_iter()).for_each(|(x, y)| {
                *x += y;
            });
            nums
        });

    let x = izip!(
        a.into_iter().rev(),
        once(0).chain(COINS.into_iter().rev().tuple_windows().map(|(x, y)| x / y)),
        nums
    )
    .fold(0, |c, (aa, r, m)| (c * r + m).saturating_sub(aa));

    if x == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
