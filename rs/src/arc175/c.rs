use std::cmp::{max, min};

use itertools::{repeat_n, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut intervals: [(usize, usize); n],
    };

    intervals.iter_mut().for_each(|(_, r)| *r += 1);

    let overlap = |(l0, r0): (usize, usize), (l1, r1): (usize, usize)| {
        if r0 <= l1 || r1 <= l0 {
            None
        } else {
            Some((max(l0, l1), min(r0, r1)))
        }
    };

    let intervals = intervals;
    let mut fixed_intervals = intervals.clone();

    let mut i0 = 0;
    while i0 + 1 < n {
        if let Some(t) = overlap(fixed_intervals[i0], fixed_intervals[i0 + 1]) {
            assert!(t.0 < t.1);
            fixed_intervals[i0 + 1] = t;
            i0 += 1;
        } else {
            break;
        }
    }

    let mut ans = if i0 == n - 1 {
        let x = fixed_intervals[i0].0;

        vec![x; n]
    } else {
        let x = if fixed_intervals[i0].1 <= fixed_intervals[i0 + 1].0 {
            fixed_intervals[i0].1 - 1
        } else {
            fixed_intervals[i0].0
        };

        repeat_n(x, i0 + 1)
            .chain(
                fixed_intervals[i0 + 1..]
                    .iter()
                    .copied()
                    .scan(x, |pos, (l, r)| {
                        if *pos < l {
                            *pos = l;
                        } else if *pos >= r {
                            *pos = r - 1;
                        }

                        Some(*pos)
                    }),
            )
            .collect::<Vec<_>>()
    };

    let mut last = ans[n - 1];
    for i in (0..n - 1).rev() {
        if last < ans[i] {
            ans[i] = max(last, intervals[i].0);
        }

        last = ans[i];
    }

    println!("{}", ans.iter().join(" "));
}
