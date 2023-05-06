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

use itertools_num::ItertoolsNum;

fn days_in_year(m: usize, d: usize) -> usize {
    let days_in_month = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let c = once(0)
        .chain(days_in_month.iter().copied())
        .cumsum::<usize>()
        .collect_vec();

    c[m - 1] + d - 1
}

// monday = 0
fn weekday(d: usize) -> usize {
    (d + 6) % 7
}

fn main() {
    let n: usize = read();

    let holidays = read_vec(n, || {
        let (m, d) = read::<String>()
            .split("/")
            .map(|p| p.parse::<usize>().unwrap())
            .next_tuple()
            .unwrap();
        days_in_year(m, d)
    });

    let is_holiday = holidays
        .iter()
        .fold(vec![false; 366], |mut is_holiday, &d| {
            is_holiday[d] = true;
            is_holiday
        });

    let ans = (0..366)
        .scan((0usize, 0usize), |(s, c), d| {
            if is_holiday[d] {
                *c += 1;
            }
            if weekday(d) >= 5 {
                *c += 1;
            }

            if *c >= 1 {
                *s += 1;
                *c -= 1;
            } else {
                *s = 0;
            }

            Some(*s)
        })
        .max()
        .unwrap();

    println!("{}", ans);
}
