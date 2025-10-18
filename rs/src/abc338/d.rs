use std::cmp::{Ordering, max, min};

use itertools::Itertools;
use itertools_num::ItertoolsNum;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, m: usize,
        x: [Usize1; m],
    };

    let (s, t) =
        x.iter()
            .copied()
            .tuple_windows()
            .fold((0, vec![0; 2 * n]), |(s, mut t), (i, j)| {
                let (i, j) = (min(i, j), max(i, j));

                let d = ((j - i) as i64) - (n + i - j) as i64;

                let s = match d.cmp(&0) {
                    Ordering::Less => {
                        t[i] += d.abs();
                        t[j] -= d.abs();

                        s + (j - i)
                    }
                    Ordering::Equal => s + (j - i),
                    Ordering::Greater => {
                        t[j] += d.abs();
                        t[n + i] -= d.abs();

                        s + (n + i - j)
                    }
                };

                (s, t)
            });

    let t = t.into_iter().cumsum::<i64>().collect::<Vec<_>>();

    let x = (0..n).map(|i| t[i] + t[i + n]).min().unwrap() as usize;

    let ans = s + x;

    println!("{ans}");
}
