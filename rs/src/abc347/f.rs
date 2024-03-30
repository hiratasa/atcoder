use std::{cmp::max, iter::once};

use itertools::{iproduct, izip};
use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        a: [[usize; n]; n],
    };

    let ans = (0..4)
        .scan(a, |a, _| {
            rotate90(a);

            let sums = once(vec![])
                .chain(a.iter().map(|row| {
                    once(0)
                        .chain(row.iter().copied())
                        .cumsum::<usize>()
                        .collect::<Vec<_>>()
                }))
                .scan(vec![0; n + 1], |sums, row| {
                    izip!(sums.iter_mut(), row.iter()).for_each(|(x, y)| *x += *y);

                    Some(sums.clone())
                })
                .collect::<Vec<_>>();

            let b = (0..=n - m)
                .map(|i| {
                    (0..=n - m)
                        .map(|j| sums[i + m][j + m] + sums[i][j] - sums[i][j + m] - sums[i + m][j])
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let mut maxs_topleft = vec![vec![0usize; n - m + 2]; n - m + 2];
            for i in 0..=n - m {
                for j in 0..=n - m {
                    maxs_topleft[i + 1][j + 1] =
                        max(b[i][j], max(maxs_topleft[i][j + 1], maxs_topleft[i + 1][j]));
                }
            }

            let mut maxs_topright = vec![vec![0usize; n - m + 2]; n - m + 2];
            for i in 0..=n - m {
                for j in (0..=n - m).rev() {
                    maxs_topright[i + 1][j] = max(
                        b[i][j],
                        max(maxs_topright[i][j], maxs_topright[i + 1][j + 1]),
                    );
                }
            }

            let mut maxs_top = vec![0usize; n - m + 2];
            for i in 0..=n - m {
                let ma = b[i].iter().copied().max().unwrap();
                maxs_top[i + 1] = max(maxs_top[i], ma);
            }

            let mut maxs_bottom = vec![0usize; n - m + 2];
            for i in (0..=n - m).rev() {
                let ma = b[i].iter().copied().max().unwrap();
                maxs_bottom[i] = max(maxs_bottom[i + 1], ma);
            }

            // パターンA:
            // 1 2
            //  3
            let ans0 = iproduct!(0..=n - 2 * m, 0..=n - 2 * m)
                .map(|(i, j)| {
                    maxs_topleft[i + 1][j + 1] + maxs_topright[i + 1][j + m] + maxs_bottom[i + m]
                })
                .max()
                .unwrap();

            // パターンB:
            //  1
            //  2
            //  3
            let ans1 = if 3 * m <= n {
                (m - 1..=n - 2 * m)
                    .map(|i| {
                        let ma = b[i].iter().copied().max().unwrap();

                        maxs_top[i + 1 - m] + ma + maxs_bottom[i + m]
                    })
                    .max()
                    .unwrap()
            } else {
                0
            };

            Some(max(ans0, ans1))
        })
        .max()
        .unwrap();

    println!("{ans}");
}

fn rotate90(a: &mut Vec<Vec<usize>>) {
    let n = a.len();

    let mut b = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            b[i][j] = a[j][n - 1 - i];
        }
    }

    *a = b;
}
