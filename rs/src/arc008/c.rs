#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

use ordered_float::NotNan;

// vec with some initial value
#[allow(unused_macros)]
macro_rules! vvec {
    ($($x:expr),+; $y:expr; $n:expr) => {{
        let mut v = vec![$y; $n];

        let mut it = v.iter_mut();
        $(
            *it.next().unwrap() = $x;
        )+

        v
    }}
}

#[allow(unused_macros)]
macro_rules! read_tuple {
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
fn read_row<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_col<T: FromStr>(n: usize) -> Vec<T> {
    (0..n).map(|_| read()).collect()
}

#[allow(dead_code)]
fn read_mat<T: FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_row()).collect()
}

#[allow(dead_code)]
fn read_vec<R, F: FnMut() -> R>(n: usize, mut f: F) -> Vec<R> {
    (0..n).map(|_| f()).collect()
}

fn main() {
    let n: usize = read();

    let xytr = read_vec(n, || read_tuple!(f64, f64, f64, f64));

    let costs = xytr
        .iter()
        .copied()
        .map(|(x1, y1, t, _)| {
            xytr.iter()
                .copied()
                .map(|(x2, y2, _, r)| {
                    let dx = x1 - x2;
                    let dy = y1 - y2;
                    let d = (dx * dx + dy * dy).sqrt();
                    d / if t < r { t } else { r }
                })
                .collect_vec()
        })
        .collect_vec();

    let mut q = BinaryHeap::new();
    let mut least_costs = vec![NotNan::new(std::f64::MAX).unwrap(); n];
    q.push(Reverse((NotNan::new(0.0).unwrap(), 0)));
    least_costs[0] = NotNan::new(0.0).unwrap();
    while let Some(Reverse((least_cost, v))) = q.pop() {
        if least_cost > least_costs[v] {
            continue;
        }

        for u in 0..n {
            let next_cost = least_cost + costs[v][u];
            if next_cost < least_costs[u] {
                least_costs[u] = next_cost;
                q.push(Reverse((next_cost, u)));
            }
        }
    }

    let ans = zip(
        least_costs.iter().copied().sorted().skip(1),
        (0..n - 1).rev(),
    )
    // .inspect(|t| eprintln!("{:?}", t))
    .map(|(c, i)| c + (i as f64))
    .max()
    .unwrap_or(NotNan::new(0.0).unwrap());
    println!("{:.10}", ans);
}
