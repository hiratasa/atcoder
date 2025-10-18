use std::iter::once;

use itertools::{iproduct, izip};
use itertools_num::ItertoolsNum;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize, w: usize, k: usize,
        s: [Chars; h],
    };

    let t = once(vec![0; w + 1])
        .chain(s.iter().map(|row| {
            once(0)
                .chain(row.iter().copied().map(|c| match c {
                    '.' => 1,
                    'o' => 0,
                    'x' => -(1i64 << 20),
                    _ => unreachable!(),
                }))
                .cumsum::<i64>()
                .collect::<Vec<_>>()
        }))
        .scan(vec![0; w + 1], |sum, row| {
            izip!(sum.iter_mut(), row.iter()).for_each(|(x, y)| *x += *y);

            Some(sum.clone())
        })
        .collect::<Vec<_>>();

    let ans = iproduct!(0..h.saturating_sub(k - 1), 0..w)
        .map(|(i, j)| (i, j, i + k, j + 1))
        .chain(iproduct!(0..h, 0..w.saturating_sub(k - 1)).map(|(i, j)| (i, j, i + 1, j + k)))
        .filter_map(|(i0, j0, i1, j1)| {
            let z = t[i1][j1] + t[i0][j0] - t[i0][j1] - t[i1][j0];

            if z < 0 { None } else { Some(z) }
        })
        .min();

    if let Some(ans) = ans {
        println!("{ans}");
    } else {
        println!("-1");
    }
}
