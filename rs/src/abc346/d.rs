use std::cmp::min;

use itertools::izip;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: Digits,
        c: [usize; n],
    };

    let left =
        izip!(s.iter().copied(), c.iter().copied()).fold(vec![[0; 2]], |mut dp, (x, cost)| {
            let prev = *dp.last().unwrap();
            let mut t = [0; 2];

            for i in 0..2 {
                t[i] = prev[1 - i];
                if i != x {
                    t[i] += cost;
                }
            }

            dp.push(t);

            dp
        });

    let right = izip!(s.iter().copied(), c.iter().copied()).rev().fold(
        vec![[0; 2]],
        |mut dp, (x, cost)| {
            let prev = *dp.last().unwrap();
            let mut t = [0; 2];

            for i in 0..2 {
                t[i] = prev[1 - i];
                if i != x {
                    t[i] += cost;
                }
            }

            dp.push(t);

            dp
        },
    );

    let ans = izip!(
        left.into_iter().skip(1),
        right.into_iter().skip(1).rev().skip(1)
    )
    .map(|(xx, yy)| min(xx[0] + yy[0], xx[1] + yy[1]))
    .min()
    .unwrap();

    println!("{ans}");
}

use proconio::source::{Readable, Source};
enum Digits {}
impl Readable for Digits {
    type Output = Vec<usize>;
    fn read<R: std::io::BufRead, S: Source<R>>(source: &mut S) -> Vec<usize> {
        source
            .next_token_unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
    }
}
