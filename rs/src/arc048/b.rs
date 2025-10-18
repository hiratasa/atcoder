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
use itertools::{Itertools, chain, iproduct, izip};
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
    let n: usize = read();

    let rh = read_vec(n, || read_tuple!(usize, usize));
    let rh = rh.into_iter().map(|(r, h)| (r, h - 1)).collect_vec();

    let rate_hands =
        rh.iter()
            .copied()
            .fold(vec![vec![0usize; 3]; 100001], |mut rate_hands, (r, h)| {
                rate_hands[r][h] += 1;
                rate_hands
            });
    let cumsum = chain(rate_hands.iter().map(|v| v[0] + v[1] + v[2]), once(0))
        .scan(0usize, |acc, s| Some(replace(acc, *acc + s)))
        .collect_vec();

    rh.iter()
        .copied()
        .map(|(r, h)| {
            let win = cumsum[r] + rate_hands[r][(h + 1) % 3];
            let lose = cumsum[cumsum.len() - 1] - cumsum[r + 1] + rate_hands[r][(h + 2) % 3];
            let draw = n - 1 - win - lose;

            (win, lose, draw)
        })
        .for_each(|(win, lose, draw)| {
            println!("{} {} {}", win, lose, draw);
        });
}
