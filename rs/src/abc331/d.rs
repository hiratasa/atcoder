use std::iter::once;

use itertools::izip;
use itertools_num::ItertoolsNum;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize, q: usize,
        p: [Chars; n],
        queries: [(usize, usize, usize, usize); q],
    };

    let p = p
        .into_iter()
        .map(|row| row.into_iter().map(|c| c == 'B').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let sums = once(vec![0; n + 1])
        .chain(p.iter().map(|row| {
            once(0)
                .chain(row.iter().map(|&x| x as usize).cumsum::<usize>())
                .collect::<Vec<_>>()
        }))
        .scan(vec![0; n + 1], |t, row| {
            izip!(t.iter_mut(), row.iter()).for_each(|(x, y)| *x += *y);

            Some(t.clone())
        })
        .collect::<Vec<_>>();

    let sum = |a: usize, b: usize, c: usize, d: usize| {
        assert!(a <= c, "{a} {c}");
        assert!(b <= d, "{b} {d}");
        sums[c][d] + sums[a][b] - sums[c][b] - sums[a][d]
    };

    let s = sums[n][n];

    dbg!(s);

    queries
        .iter()
        .map(|&(a, b, c, d)| {
            let c = c + 1;
            let d = d + 1;

            if a / n == c / n {
                if b / n == d / n {
                    sum(a % n, b % n, c % n, d % n)
                } else {
                    sum(a % n, b % n, c % n, n)
                        + (d / n - b / n - 1) * sum(a % n, 0, c % n, n)
                        + sum(a % n, 0, c % n, d % n)
                }
            } else {
                if b / n == d / n {
                    sum(a % n, b % n, n, d % n)
                        + (c / n - a / n - 1) * sum(0, b % n, n, d % n)
                        + sum(0, b % n, c % n, d % n)
                } else {
                    sum(a % n, b % n, n, n)
                        + sum(a % n, 0, n, d % n)
                        + sum(0, b % n, c % n, n)
                        + sum(0, 0, c % n, d % n)
                        + (d / n - b / n - 1) * (sum(a % n, 0, n, n) + sum(0, 0, c % n, n))
                        + (c / n - a / n - 1) * (sum(0, b % n, n, n) + sum(0, 0, n, d % n))
                        + (c / n - a / n - 1) * (d / n - b / n - 1) * s
                }
            }
        })
        .for_each(|ans| {
            println!("{ans}");
        })
}
