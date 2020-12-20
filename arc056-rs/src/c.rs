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

fn main() {
    let (n, k) = read_cols!(usize, i64);

    let w = (0..n).map(|_| read_vec::<i64>()).collect::<Vec<_>>();

    let score = (0..(1 << n))
        .map(|u| {
            let it = (0..n).filter(|i| (u & (1 << i)) > 0);

            it.clone()
                .map(|i| it.clone().map(|j| w[i][j]).sum::<i64>())
                .sum::<i64>()
                / 2
        })
        .collect::<Vec<_>>();
    let total_w = score[(1 << n) - 1];

    let ans = (1..(1 << n)).fold(vec![0], |mut s, u: usize| {
        s.push(
            std::iter::successors(Some(u), |&t| t.checked_sub(1).map(|t| t & u))
                .skip(1)
                .map(|t| {
                    let r = u & !t;

                    s[t] + k + score[r]
                })
                .max()
                .unwrap(),
        );

        s
    })[(1 << n) - 1]
        - total_w;
    println!("{}", ans);
}
