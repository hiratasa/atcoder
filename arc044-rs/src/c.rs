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
use itertools::{chain, iproduct, izip, Itertools};
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
    let (w, h, q) = read_tuple!(usize, usize, usize);

    let tdx = read_vec(q, || read_tuple!(usize, usize, usize));

    let dims = [w, h];
    let ans = tdx
        .iter()
        .copied()
        .sorted_by_key(|&(t, d, x)| (d, t, x))
        .group_by(|(_t, d, _x)| *d)
        .into_iter()
        .map(|(d, it)| {
            it.map(|(t, _, x)| (t, x - 1))
                .group_by(|(t, _x)| *t)
                .into_iter()
                .fold(vec![0usize; dims[d]], |mut dp, (_t, it2)| {
                    let xs = it2.map(|(_t, x)| x).collect_vec();
                    xs.iter()
                        .copied()
                        .filter(|&x| x < dims[d] - 1)
                        .for_each(|x| {
                            dp[x + 1] = min(dp[x + 1], dp[x].saturating_add(1));
                        });
                    xs.iter().copied().rev().filter(|&x| x > 0).for_each(|x| {
                        dp[x - 1] = min(dp[x - 1], dp[x].saturating_add(1));
                    });
                    xs.iter().copied().for_each(|x| dp[x] = usize::MAX);

                    dp
                })
                .into_iter()
                .min()
                .unwrap()
        })
        .fold(0usize, |acc, s| acc.saturating_add(s));
    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
