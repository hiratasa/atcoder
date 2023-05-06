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
use itertools::{chain, iproduct, izip, unfold, Itertools};
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
fn read_vec_itr<R, F: FnMut() -> R>(n: usize, mut f: F) -> impl Iterator<Item = R> {
    (0..n).map(move |_| f())
}

#[allow(dead_code)]
fn read_vec<R, F: FnMut() -> R>(n: usize, f: F) -> Vec<R> {
    read_vec_itr(n, f).collect()
}

fn main() {
    let (h, w) = read_tuple!(usize, usize);

    let a = once(vec![0; w + 2])
        .chain(read_vec_itr(h, || {
            once(0)
                .chain(read_row::<usize>())
                .chain(once(0))
                .collect_vec()
        }))
        .chain(once(vec![0; w + 2]))
        .collect_vec();

    let b = a
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &aa)| (aa, i, j)))
        .sorted_by_key(|&t| Reverse(t))
        .collect_vec();

    const M: usize = 1_000_000_007;
    let ans = b
        .iter()
        .copied()
        .take_while(|&(aa, _, _)| aa > 0)
        .scan(vec![vec![0usize; w + 2]; h + 2], |dp, (aa, i, j)| {
            let deltas: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

            dp[i][j] = 1 + deltas
                .iter()
                .map(|(di, dj)| ((i as i64 + di) as usize, (j as i64 + dj) as usize))
                .filter(|&(ni, nj)| a[ni][nj] > aa)
                .map(|(ni, nj)| dp[ni][nj])
                .fold(0usize, |acc, d| (acc + d) % M);
            Some(dp[i][j])
        })
        .fold(0usize, |acc, d| (acc + d) % M);
    println!("{}", ans);
}
