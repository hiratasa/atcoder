use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        trains: [(usize, usize, usize, usize, usize, usize); m],
    };

    let by_stations = trains
        .into_iter()
        .into_group_map_by(|&(_, _, _, _, _, b)| b);

    let mut q = BinaryHeap::new();
    q.push((usize::MAX, n));
    let mut ans = vec![0; n + 1];
    while let Some((t, x)) = q.pop() {
        if t < ans[x] {
            continue;
        }
        ans[x] = t;

        for &(l, d, k, c, a, _b) in by_stations
            .get(&x)
            .map_or([].as_slice(), |v| v.as_slice())
            .iter()
        {
            // l+c ～ l+c+(k-1)d
            if t < l + c {
                continue;
            }

            let i = ((t - l - c) / d).min(k - 1);

            q.push((l + i * d, a));
        }
    }

    for i in 1..n {
        if ans[i] == 0 {
            println!("Unreachable");
        } else {
            println!("{}", ans[i]);
        }
    }
}
