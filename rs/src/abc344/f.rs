use std::cmp::{Reverse, min};

use itertools::{Itertools, iproduct};
use proconio::input;

fn main() {
    input! {
        n: usize,
        parking: [[usize; n]; n],
        right: [[usize; n - 1]; n],
        down: [[usize; n]; n - 1],
    };

    let mut table = vec![vec![vec![vec![(usize::MAX, Reverse(usize::MAX)); n]; n]; n]; n];
    table[0][0][0][0] = (0, Reverse(0));
    for i in 0..n {
        for j in 0..n {
            for i0 in 0..=i {
                for j0 in 0..=j {
                    let (k, Reverse(c)) = table[i][j][i0][j0];
                    let p = parking[i0][j0];

                    // right
                    if j + 1 < n {
                        let cost = right[i][j];
                        let s = (cost.saturating_sub(c) + p - 1) / p;

                        let (i1, j1) = if parking[i][j + 1] > p {
                            (i, j + 1)
                        } else {
                            (i0, j0)
                        };

                        table[i][j + 1][i1][j1] =
                            min(table[i][j + 1][i1][j1], (k + s, Reverse(c + s * p - cost)));
                    }

                    // down
                    if i + 1 < n {
                        let cost = down[i][j];
                        let s = (cost.saturating_sub(c) + p - 1) / p;

                        let (i1, j1) = if parking[i + 1][j] > p {
                            (i + 1, j)
                        } else {
                            (i0, j0)
                        };

                        table[i + 1][j][i1][j1] =
                            min(table[i + 1][j][i1][j1], (k + s, Reverse(c + s * p - cost)));
                    }
                }
            }
        }
    }

    let ans = 2 * n - 2
        + iproduct!(0..n, 0..n)
            .map(|(i, j)| table[n - 1][n - 1][i][j].0)
            .min()
            .unwrap();

    println!("{ans}");
}
