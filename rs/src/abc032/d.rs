#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::iter::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, izip, repeat_n};
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;

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
        io::stdin().read_line(&mut line).unwrap();

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
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_row<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

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
    let (n, cap) = read_tuple!(usize, usize);

    let vw = read_vec(n, || read_tuple!(usize, usize));

    let max_v = vw.iter().copied().map(|(v, _)| v).max().unwrap();
    let max_w = vw.iter().copied().map(|(_, w)| w).max().unwrap();

    let ans = if n <= 30 {
        let mut s = vw
            .iter()
            .copied()
            .take(n / 2)
            .fold(vec![(0usize, 0usize)], |mut s, (v, w)| {
                s.append(
                    &mut s
                        .iter()
                        .copied()
                        .map(|(ww, vv)| (ww + w, vv + v))
                        .collect_vec(),
                );
                s
            });

        s.sort();

        let m = s
            .iter()
            .copied()
            .fold((BTreeMap::new(), 0usize), |(mut m, c), (ww, vv)| {
                m.insert(ww, max(c, vv));

                (m, max(c, vv))
            })
            .0;

        let s2 = vw
            .iter()
            .copied()
            .skip(n / 2)
            .fold(vec![(0usize, 0usize)], |mut s2, (v, w)| {
                s2.append(
                    &mut s2
                        .iter()
                        .copied()
                        .map(|(ww, vv)| (ww + w, vv + v))
                        .collect_vec(),
                );
                s2
            });

        s2.iter()
            .copied()
            .filter(|&(ww, _vv)| ww <= cap)
            .map(|(ww, vv)| vv + m.range(..=(cap - ww)).next_back().unwrap().1)
            .max()
            .unwrap()
    } else if max_w <= 1000 {
        let cap = min(cap, max_w * n);
        vw.iter().copied().fold(vec![0; cap + 1], |prev, (v, w)| {
            izip!(
                prev.iter().copied(),
                itertools::repeat_n(0, w).chain(prev.iter().copied().map(|vv| vv + v)),
            )
            .map(|(v1, v2)| v1.max(v2))
            .collect_vec()
        })[cap]
    } else if max_v <= 1000 {
        let max_total_v = max_v * n;
        vw.iter()
            .copied()
            .fold(
                vvec![0; std::usize::MAX; max_total_v + 1],
                |prev, (v, w)| {
                    izip!(
                        prev.iter().copied(),
                        itertools::repeat_n(w, v)
                            .chain(prev.iter().copied().map(|ww| ww.saturating_add(w))),
                    )
                    .map(|(v1, v2)| v1.min(v2))
                    .collect_vec()
                },
            )
            .into_iter()
            .enumerate()
            .rev()
            .find(|&(_v, w)| w <= cap)
            .unwrap()
            .0
    } else {
        unreachable!()
    };
    println!("{}", ans);
}
