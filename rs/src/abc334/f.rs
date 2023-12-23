use std::{collections::VecDeque, iter::once};

use itertools::Itertools;
use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        s: (f64, f64),
        xy: [(f64, f64); n],
    };

    let distance =
        |(x0, y0): (f64, f64), (x1, y1): (f64, f64)| ((x0 - x1).powi(2) + (y0 - y1).powi(2)).sqrt();

    let dists = once(0.0)
        .chain(
            xy.iter()
                .copied()
                .tuple_windows()
                .map(|(p0, p1)| distance(p0, p1)),
        )
        .cumsum::<f64>()
        .collect::<Vec<_>>();

    let q = (0..n).fold(
        once((0, distance(s, xy[0]))).collect::<VecDeque<_>>(),
        |mut mins, i| {
            while matches!(mins.front(), Some(&(j, _)) if j + k <= i) {
                mins.pop_front();
            }

            let (i_min, mi) = mins.front().unwrap();
            let z = mi + dists[i];

            if i < n - 1 {
                let a = z + distance(xy[i], s) + distance(s, xy[i + 1]) - dists[i + 1];
                while matches!(mins.back(), Some(&(_, b)) if b >= a) {
                    mins.pop_back();
                }

                mins.push_back((i + 1, a));
            } else {
                mins.push_back((i + 1, z + distance(xy[i], s)))
            }

            mins
        },
    );

    let ans = q.back().unwrap().1;

    println!("{ans}");
}
