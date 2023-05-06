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

fn add(lhs: &mut usize, rhs: usize) -> usize {
    *lhs += rhs;
    *lhs
}

fn main() {
    let n: usize = read();

    let a = read_row::<usize>();

    let cumsum_left = chain(
        once(0),
        a.iter().copied().scan(0usize, |acc, aa| Some(add(acc, aa))),
    )
    .collect_vec();

    let cumsum_cumsum_left = cumsum_left
        .iter()
        .copied()
        .scan(0usize, |acc, aa| Some(add(acc, aa)))
        .collect_vec();

    let cumsum_right = cumsum_left
        .iter()
        .copied()
        .map(|c| cumsum_left[n] - c)
        .collect_vec();

    let mut cumsum_cumsum_right = cumsum_right
        .iter()
        .copied()
        .rev()
        .scan(0usize, |acc, aa| Some(add(acc, aa)))
        .collect_vec();
    cumsum_cumsum_right.reverse();
    let cumsum_cumsum_right = cumsum_cumsum_right;

    let ans = (0..n)
        .filter(|&i| i % 2 == 0)
        .map(|i| {
            cumsum_cumsum_left[i]
                + i * i.wrapping_sub(1) / 2
                + cumsum_cumsum_right[i + 1]
                + (n - i - 1) * n.wrapping_sub(i + 2) / 2
        })
        .min()
        .unwrap();

    println!("{}", ans);
}
