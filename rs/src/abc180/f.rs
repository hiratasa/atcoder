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

const M: usize = 1000000007;

fn main() {
    let (n, m, l) = read_cols!(usize, usize, usize);

    if l == 1 {
        println!("{}", 0);
        return;
    }

    let mut fact = vec![1; n + 1];
    let mut inv_fact = vec![1; n + 1];
    let mut inv = vec![1; n + 1];
    for i in 2..=n {
        fact[i] = fact[i - 1] * i % M;
        inv[i] = (M - (M / i) * inv[M % i] % M) % M;
        inv_fact[i] = inv_fact[i - 1] * inv[i] % M;
    }

    let combi = |n: usize, k: usize| fact[n] * inv_fact[k] % M * inv_fact[n - k] % M;

    let loop_combi = |n: usize| {
        if n == 2 { 1 } else { fact[n - 1] * inv[2] % M }
    };

    let line_combi = |n: usize| {
        if n == 1 { 1 } else { fact[n] * inv[2] % M }
    };

    let k = n - m;
    let dp = (0..=n).fold(vec![vec![vec![0; k + 1]; n + 1]; 2], |mut dp, nn| {
        if nn == 0 {
            dp[0][0][0] = 1;
        } else {
            for kk in 0..=k {
                // loop
                if nn >= 2 {
                    for i in 2..=min(l - 1, nn) {
                        dp[0][nn][kk] += dp[0][nn - i][kk] * combi(n - (nn - i) - 1, i - 1) % M
                            * loop_combi(i)
                            % M;
                    }
                    for i in 2..=min(l, nn) {
                        dp[1][nn][kk] += dp[1][nn - i][kk] * combi(n - (nn - i) - 1, i - 1) % M
                            * loop_combi(i)
                            % M;
                    }
                }

                // non-loop
                if kk > 0 {
                    for i in 1..=min(l - 1, nn) {
                        dp[0][nn][kk] += dp[0][nn - i][kk - 1] * combi(n - (nn - i) - 1, i - 1) % M
                            * line_combi(i)
                            % M;
                    }
                    for i in 1..=min(l, nn) {
                        dp[1][nn][kk] += dp[1][nn - i][kk - 1] * combi(n - (nn - i) - 1, i - 1) % M
                            * line_combi(i)
                            % M;
                    }
                }

                if nn >= l {
                    // loop
                    dp[1][nn][kk] +=
                        dp[0][nn - l][kk] * combi(n - (nn - l) - 1, l - 1) % M * loop_combi(l) % M;

                    // non-loop
                    if kk > 0 {
                        dp[1][nn][kk] += dp[0][nn - l][kk - 1] * combi(n - (nn - l) - 1, l - 1) % M
                            * line_combi(l)
                            % M;
                    }
                }

                dp[0][nn][kk] %= M;
                dp[1][nn][kk] %= M;
            }
        }
        dp
    });

    println!("{}", dp[1][n][k]);
}
