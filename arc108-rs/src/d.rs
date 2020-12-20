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
macro_rules! read_cols {
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
fn read_vec<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

fn main() {
    let n: usize = read();

    let caa: char = read();
    let cab: char = read();
    let cba: char = read();
    let cbb: char = read();

    const M: usize = 1_000_000_007;

    let fact: Vec<_> = std::iter::once(1)
        .chain((1..=n).scan(1, |f, i| {
            *f = *f * i % M;
            Some(*f)
        }))
        .collect();
    let inv = (2..=n).fold(vec![1, 1], |mut inv, i| {
        inv.push((M - (M / i) * inv[M % i] % M) % M);
        inv
    });
    let inv_fact: Vec<_> = inv
        .iter()
        .scan(1, |f, i| {
            *f = *f * i % M;
            Some(*f)
        })
        .collect();

    let ans = if n == 2 {
        1
    } else if cab == 'A' {
        if caa == 'A' {
            1
        } else if cba == 'A' {
            (1..=n / 2)
                .map(|i| fact[n - i - 1] * inv_fact[i - 1] % M * inv_fact[n - 2 * i] % M)
                .fold(0usize, |acc, a| (acc + a) % M)
        } else {
            (0..n - 3).fold(1usize, |p, _| p * 2 % M)
        }
    } else {
        if cbb == 'B' {
            1
        } else if cba == 'B' {
            (1..=n / 2)
                .map(|i| fact[n - i - 1] * inv_fact[i - 1] % M * inv_fact[n - 2 * i] % M)
                .fold(0usize, |acc, a| (acc + a) % M)
        } else {
            (0..n - 3).fold(1usize, |p, _| p * 2 % M)
        }
    };

    println!("{}", ans);
}
