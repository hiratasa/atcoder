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
use std::io;
#[allow(unused_imports)]
use std::iter::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

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
    let t = read::<String>();

    const K: usize = 10_000_000_000;

    let ans = if t.len() == 1 {
        match t.as_str() {
            "0" => K,
            "1" => 2 * K,
            _ => unreachable!(),
        }
    } else if t.len() == 2 {
        match t.as_str() {
            "00" => 0,
            "01" => K - 1,
            "10" => K,
            "11" => K,
            _ => unreachable!(),
        }
    } else {
        match t.split_at(3).0 {
            "110" => {
                if t.chars().chunks(3).into_iter().all(|chunk| {
                    let p = chunk.collect::<String>();
                    "110".starts_with(&p)
                }) {
                    let m = (n + 2) / 3;
                    K - m + 1
                } else {
                    0
                }
            }
            "101" => {
                if t.chars().skip(2).chunks(3).into_iter().all(|chunk| {
                    let p = chunk.collect::<String>();
                    "110".starts_with(&p)
                }) {
                    let m = (n - 2 + 2) / 3 + 1;
                    K - m + 1
                } else {
                    0
                }
            }
            "011" => {
                if t.chars().skip(1).chunks(3).into_iter().all(|chunk| {
                    let p = chunk.collect::<String>();
                    "110".starts_with(&p)
                }) {
                    let m = (n - 1 + 2) / 3 + 1;
                    K - m + 1
                } else {
                    0
                }
            }
            _ => 0,
        }
    };

    println!("{}", ans);
}
