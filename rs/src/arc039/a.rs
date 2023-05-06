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
    let (a, b) = read_cols!(i64, i64);

    let ans_a = [1, 10, 100]
        .iter()
        .map(|&d| (d, a % (d * 10) / d))
        .map(|(d, c)| a - c * d + 9 * d - b)
        .max()
        .unwrap();

    let ans_b = [1, 10]
        .iter()
        .map(|&d| (d, b % (d * 10) / d))
        .map(|(d, c)| a - (b - c * d))
        .max()
        .unwrap();

    let ans_b2 = a - (b - b / 100 * 100 + 100);

    let ans = [a - b, ans_a, ans_b, ans_b2].iter().copied().max().unwrap();
    println!("{}", ans);
}
