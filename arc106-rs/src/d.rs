#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::cmp::*;
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
    // sum_l sum_r(>l) (a_l + a_r)^x
    // = sum_l sum_r(>l) sum_i C(x, i) a_l^i a_r^(x-i)
    // = sum_i C(x,i) sum_l a_l^i sum_r(>l) a_r^(x-i)
    let (n, k) = read_cols!(usize, usize);

    let a = read_vec::<usize>();

    let mut fact = vec![1; k + 1];
    let mut inv_fact = vec![1; k + 1];
    let mut inv = vec![1; k + 1];
    let mut pow2 = vec![1; k + 1];
    pow2[1] = 2;
    for i in 2..=k {
        fact[i] = fact[i - 1] * i % M;
        inv[i] = (M - (M / i) * inv[M % i] % M) % M;
        inv_fact[i] = inv_fact[i - 1] * inv[i] % M;
        pow2[i] = pow2[i - 1] * 2 % M;
    }

    let sums = (0..=k)
        .scan(vec![1; n], |st, _| {
            let s = st.iter().fold(0, |acc, ss| (acc + ss) % M);

            st.iter_mut()
                .zip(a.iter())
                .for_each(|(bb, aa)| *bb = *bb * aa % M);

            Some(s)
        })
        .collect::<Vec<_>>();

    for x in 1..=k {
        let ans = ((0..=x)
            .map(|i| {
                fact[x] * inv_fact[i] % M * inv_fact[x - i] % M * sums[i] % M * sums[x - i] % M
            })
            .fold(0, |acc, t| (acc + t) % M)
            + M
            - pow2[x] * sums[x] % M)
            * inv[2]
            % M;
        println!("{}", ans % M);
    }
}
