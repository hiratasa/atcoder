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
use itertools::{chain, iproduct, iterate, izip, Itertools};
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

trait SliceCopiedExt<T> {
    fn citer(&self) -> std::iter::Copied<std::slice::Iter<T>>;
}

impl<V, T> SliceCopiedExt<T> for V
where
    V: std::ops::Deref<Target = [T]>,
    T: Copy,
{
    fn citer(&self) -> std::iter::Copied<std::slice::Iter<T>> {
        self.iter().copied()
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn solve(n: usize, s: usize, k: usize) -> Option<usize> {
    assert!(s < n);
    if s == 0 {
        Some(0)
    } else if k == 0 {
        None
    } else if (n - s) % k == 0 {
        Some((n - s) / k)
    } else {
        let g = gcd(n, k);
        if let Some(r) = solve(k, s % k, n % k) {
            Some((n / k - s / k + r * (n / k)) % (n / g))
        } else {
            None
        }
    }
}

fn main() {
    let t: usize = read();

    let nsk = read_vec(t, || read_tuple!(usize, usize, usize));

    for &(n, s, k) in &nsk {
        if let Some(ans) = solve(n, s, k) {
            println!("{}", ans);
        } else {
            println!("-1");
        }
    }
}
