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

const M: usize = 998244353;

fn main() {
    let (n, m) = read_cols!(usize, usize);

    let l = n + 2 * m;
    let mut fact = vec![1; l];
    let mut inv_fact = vec![1; l];
    let mut inv = vec![1; l];
    for i in 2..l {
        fact[i] = fact[i - 1] * i % M;
        inv[i] = (M - (M / i) * inv[M % i] % M) % M;
        inv_fact[i] = inv_fact[i - 1] * inv[i] % M;
    }

    let combi = |n: usize, k: usize| fact[n] * inv_fact[k] % M * inv_fact[n - k] % M;

    let divide_combi = |n: usize, k: usize| combi(n + k - 1, k - 1);

    let cum_divide_combi: Vec<_> = std::iter::once(0)
        .chain((0..m).scan(0, |state, i| {
            *state += divide_combi(i, n - 1) % M;
            *state %= M;
            Some(*state)
        }))
        .collect();

    let ans = (0..=m)
        .filter(|&i| (m - i) % 2 == 0)
        .take_while(|&i| i <= n)
        .map(|i| {
            combi(n, i)
                * (divide_combi((3 * m - i) / 2, n) + M
                    - (n - i) * cum_divide_combi[(m - i) / 2] % M
                    + M
                    - i * cum_divide_combi[(m - i + 2) / 2] % M)
                % M
        })
        .fold(0, |acc, x| (acc + x) % M);
    println!("{}", ans);
}
