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

fn consistent(s1: &String, s2: &String) -> bool {
    s1.chars()
        .zip(s2.chars())
        .all(|(c1, c2)| c1 == c2 || c1 == '*' || c2 == '*')
}

fn main() {
    let (n, _m) = read_cols!(usize, usize);

    let p = (0..n).map(|_| read::<String>()).collect::<Vec<_>>();

    let oks = (0..n)
        .flat_map(|i| {
            let p = &p;
            (0..n)
                .filter(move |j| consistent(&p[i], &p[*j]))
                .map(move |j| (i, j))
        })
        .collect::<BTreeSet<_>>();

    let groups = (0..1 << n)
        .map(|s| {
            (0..n).filter(|i| s & (1 << i) > 0).all(|i| {
                (0..n)
                    .filter(|j| s & (1 << j) > 0)
                    .all(|j| oks.contains(&(i, j)))
            })
        })
        .collect::<Vec<_>>();

    let ans = (1..1 << n).fold(vec![0usize], |mut dp, s: usize| {
        dp.push(
            std::iter::successors(Some(s), |t| t.checked_sub(1).map(|t| t & s))
                .skip(1)
                .filter(|&t| groups[s & !t])
                .filter_map(|t| dp[t].checked_add(1))
                .min()
                .unwrap_or(std::usize::MAX),
        );
        dp
    })[(1 << n) - 1];

    println!("{}", ans);
}
