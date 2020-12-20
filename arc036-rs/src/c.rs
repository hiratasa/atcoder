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

const M: usize = 1_000_000_007;

fn main() {
    let (_n, k) = read_cols!(usize, usize);

    let s = read::<String>();

    let ans = s
        .chars()
        .fold(
            vvec![vvec![1; 0; k + 1]; vec![0usize; k + 1]; k + 1],
            |prev: Vec<Vec<usize>>, c| {
                (0..=k)
                    .map(|u| {
                        (0..=k)
                            .map(|p| {
                                let a1 = if c == '0' || c == '?' {
                                    if p == 0 && u >= 1 {
                                        (prev[u][p + 1] + prev[u - 1][p]) % M
                                    } else if p < k {
                                        prev[u][p + 1]
                                    } else {
                                        0
                                    }
                                } else {
                                    0
                                };

                                let a2 = if (c == '1' || c == '?') && p <= u {
                                    if p == u && u >= 1 {
                                        (prev[u][p - 1] + prev[u - 1][p - 1]) % M
                                    } else if p >= 1 {
                                        prev[u][p - 1]
                                    } else {
                                        0
                                    }
                                } else {
                                    0
                                };

                                (a1 + a2) % M
                            })
                            .collect()
                    })
                    .collect()
            },
        )
        .into_iter()
        .fold(0, |acc, dp| {
            (acc + dp.into_iter().fold(0, |acc, d| (acc + d) % M)) % M
        });
    println!("{}", ans);
}
