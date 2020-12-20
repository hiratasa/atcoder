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

fn gcm(a: usize, b: usize) -> usize {
    if a == 0 {
        return b;
    }

    gcm(b % a, a)
}

// x^y mod m
fn modpow(x: usize, y: usize, m: usize) -> usize {
    if y == 0 {
        1
    } else {
        let z = modpow(x, y / 2, m);

        if y % 2 == 0 {
            z * z % m
        } else {
            z * z % m * x % m
        }
    }
}

// sum[i=0 to y-1] x^i mod m
fn modpowsum(x: usize, y: usize, m: usize) -> usize {
    if y == 0 {
        0
    } else if y == 1 {
        1
    } else {
        let z = modpowsum(x, y / 2, m);
        let w = z * (modpow(x, y / 2, m) + 1) % m;

        if y % 2 == 0 {
            w
        } else {
            (w * x % m + 1) % m
        }
    }
}

fn main() {
    let (a, b, m) = read_tuple!(usize, usize, usize);

    let g = gcm(a, b);

    let aa = modpowsum(modpow(10, g, m), a / g, m);
    let bb = modpowsum(10, b, m);
    let ans = aa * bb % m;

    println!("{}", ans);
}
