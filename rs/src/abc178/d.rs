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
fn read_vec<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

const M: i64 = 1000000007;

fn main() {
    let s: i64 = read();

    let mut fact = vec![1, 1];
    let mut ifact = vec![1, 1];
    let mut inv = vec![1, 1];
    for i in 2..=4000i64 {
        fact.push(fact.last().unwrap() * i % M);
        inv.push((M - (M / i) * inv[(M % i) as usize] % M) % M);
        ifact.push(ifact.last().unwrap() * inv.last().unwrap() % M);
    }

    let combi =
        |n: i64, m: i64| fact[n as usize] * ifact[(n - m) as usize] % M * ifact[m as usize] % M;

    let ans = (1..=2000)
        .take_while(|n| 3 * n <= s)
        .map(|n| {
            let ss = s - n * 3;

            // println!("{} {}", n, combi(ss + n - 1, n - 1));
            combi(ss + n - 1, n - 1)
        })
        .fold(0, |acc, a| (acc + a) % M);

    println!("{}", ans);
}
