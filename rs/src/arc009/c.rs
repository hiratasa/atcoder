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

const M: usize = 1777777777;

fn main() {
    let (n, k) = read_cols!(usize, usize);

    let inv = (2..=k).fold(vec![1, 1], |mut inv, i| {
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

    // C(n, k) * sum_r[r=0 to k] (-1)^r C(k, r) (k-r)!
    // = n!/(n-k)! * sum_r[r=0 to k] (-1)^r /r!
    let ans = (n - k + 1..=n).fold(1usize, |p, i| p * (i % M) % M)
        * (0..=k)
            .map(|r| {
                if r % 2 == 0 {
                    inv_fact[r]
                } else {
                    M - inv_fact[r]
                }
            })
            .fold(0usize, |acc, a| (acc + a) % M)
        % M;

    println!("{}", ans);
}
