#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_cols<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

fn main() {
    let s: Vec<_> = read::<String>().chars().collect();
    let k: usize = read();
    let n = s.len();

    let mut dp = vec![vec![vec![0; k + 1]; n + 1]; n];
    for i in 1..n {
        for j in (i..n).rev() {
            for kk in 0..=k {
                if s[i - 1] == s[j] {
                    dp[i][j][kk] = dp[i - 1][j + 1][kk] + 1;
                } else {
                    dp[i][j][kk] = max(dp[i - 1][j][kk], dp[i][j + 1][kk]);
                    if kk > 0 {
                        dp[i][j][kk] = max(dp[i][j][kk], dp[i - 1][j + 1][kk - 1] + 1);
                    }
                }
            }
        }
    }

    let ans = dp
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let a1 = 2 * t[i].iter().max().unwrap();
            let a2 = 2 * t[i + 1].iter().max().unwrap() + 1;
            max(a1, a2)
        })
        .max()
        .unwrap();
    println!("{}", ans);
}
