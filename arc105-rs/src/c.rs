#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

use permutohedron::heap_recursive;

#[allow(unused_macros)]
macro_rules! read_cols {
    ($($t:ty),+) => {{
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();

        let mut it = line.trim()
            .split_whitespace();

        ($(
            it.next().unwrap().parse::<$t>().ok().unwrap()
        ),+)
    }}
}

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_vec<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

fn main() {
    let (n, m) = read_cols!(usize, usize);

    let mut weights = read_vec::<usize>();
    weights.sort();

    let bridges = {
        let mut bridges = (0..m)
            .map(|_| read_cols!(usize, usize))
            .map(|(l, v)| (v, l))
            .collect::<Vec<_>>();
        bridges.sort();
        bridges
    };

    if weights[n - 1] > bridges[0].0 {
        println!("-1");
        return;
    }

    let max_bridges = std::iter::once((0, 0)).chain(bridges.iter().copied()).scan((0, 0), |state, (v, l)| {
        state.0 = v;
        state.1 = max(state.1, l);

        Some(*state)
    }).collect::<Vec<_>>();

    let mut ans = usize::MAX;
    heap_recursive(&mut weights, |weights| {
        let cum_weights = std::iter::once(0)
            .chain(weights.iter().copied())
            .scan(0, |cum, w| {
                *cum += w;
                Some(*cum)
            })
            .collect::<Vec<_>>();

        let mut dist = vec![0; n];
        for i in 0..n {
            for j in i + 1..n {
                let w = cum_weights[j + 1] - cum_weights[i];

                let idx = max_bridges.binary_search(&(w, 0)).unwrap_err() - 1;
                dist[j] = max(dist[j], dist[i] + max_bridges[idx].1);
            }
        }

        ans = min(ans, dist[n - 1] - dist[0]);
    });

    println!("{}", ans);
}
