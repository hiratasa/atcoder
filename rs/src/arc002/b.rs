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

fn is_leap_year(y: usize) -> bool {
    y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)
}

fn next_date(y: usize, m: usize, d: usize) -> (usize, usize, usize) {
    let months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let last_day_of_month = if m == 2 {
        if is_leap_year(y) { 29 } else { 28 }
    } else {
        months[m - 1]
    };

    if d == last_day_of_month {
        if m == 12 {
            (y + 1, 1, 1)
        } else {
            (y, m + 1, 1)
        }
    } else {
        (y, m, d + 1)
    }
}

fn main() {
    let s: String = read();

    let (y, m, d) = s
        .split("/")
        .map(|p| p.parse::<usize>().unwrap())
        .next_tuple()
        .unwrap();

    let ans = successors(Some((y, m, d)), |&(y, m, d)| Some(next_date(y, m, d)))
        .find(|&(y, m, d)| y % m == 0 && (y / m) % d == 0)
        .unwrap();
    println!("{}/{:02}/{:02}", ans.0, ans.1, ans.2);
}
