#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::*;
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
        stdin().read_line(&mut line).unwrap();

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
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_row<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

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

trait IteratorDpExt: Iterator + Sized {
    fn dp<T, F: FnMut(&Vec<T>, Self::Item) -> T>(self, init: Vec<T>, mut f: F) -> Vec<T> {
        self.fold(init, |mut dp, item| {
            let next = f(&dp, item);
            dp.push(next);
            dp
        })
    }
}

impl<I> IteratorDpExt for I where I: Iterator + Sized {}

fn main() {
    let n: usize = read();

    let a = read_row::<usize>();

    const M: usize = 1_000_000_007;

    let fact = std::iter::once(1)
        .chain((1..=n).scan(1, |f, i| {
            *f = *f * i % M;
            Some(*f)
        }))
        .collect_vec();

    let rfact = std::iter::once(1)
        .chain((1..=n).rev().scan(1, |f, i| {
            *f = *f * i % M;
            Some(*f)
        }))
        .collect_vec();

    let mfact = std::iter::once(0)
        .chain((1..=n).map(|i| fact[i - 1] * rfact[n - i] % M))
        .collect_vec();

    let cmfact = mfact
        .iter()
        .copied()
        .scan(0, |acc, m| {
            *acc += m;
            *acc %= M;
            Some(*acc)
        })
        .collect_vec();

    let ans = a
        .iter()
        .copied()
        .enumerate()
        .map(|(i, aa)| aa * (cmfact[i + 1] + cmfact[n - i] + M - cmfact[1]) % M)
        .fold(0, |acc, b| (acc + b) % M);
    println!("{}", ans);
}
