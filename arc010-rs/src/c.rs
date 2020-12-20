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
    let (_n, m, y, z) = read_cols!(usize, usize, i64, i64);

    let p = (0..m)
        .map(|i| (i, read_cols!(char, i64)))
        .map(|(i, (c, p))| (c, (i, p)))
        .collect::<HashMap<_, _>>();
    let b = read::<String>();

    let mut dp0 = vec![vec![std::i32::MIN as i64; m + 1]; 1 << m];
    dp0[0][m] = 0;
    let dp = b.chars().fold(dp0, |prev: Vec<Vec<i64>>, bb| {
        let (color, pp) = *p.get(&bb).unwrap();

        let mut dp = vec![vec![std::i32::MIN as i64; m + 1]; 1 << m];
        for s in 0..1 << m {
            for l in 0..m + 1 {
                dp[s][l] = max(dp[s][l], prev[s][l]);

                if l == color {
                    dp[s | (1 << color)][color] =
                        max(dp[s | (1 << color)][color], prev[s][l] + pp + y);
                } else {
                    dp[s | (1 << color)][color] = max(dp[s | (1 << color)][color], prev[s][l] + pp);
                }
            }
        }

        dp
    });

    let ans = dp
        .iter()
        .map(|d| d.iter().max().unwrap())
        .copied()
        .enumerate()
        .map(|(s, a)| if s == (1 << m) - 1 { a + z } else { a })
        .max()
        .unwrap();

    println!("{}", ans);
}
