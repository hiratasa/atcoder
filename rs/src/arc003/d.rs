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

use rand::distr::Distribution;
use rand::{Rng, SeedableRng};

fn main() {
    let start = std::time::Instant::now();

    let (n, m, k) = read_cols!(usize, usize, usize);

    let ab = (0..m).map(|_| read_cols!(usize, usize)).collect::<Vec<_>>();

    let mut rng = rand::rngs::SmallRng::from_os_rng();

    let dist0 = rand::distr::Uniform::new(0, n).unwrap();
    let dist1 = rand::distr::Uniform::new(0, n - 1).unwrap();

    let mut s = 0usize;
    let mut t = 0usize;
    while start.elapsed().as_millis() < 9900 {
        let v0 = (0..n).collect::<Vec<_>>();

        let v = (0..k).fold(v0, |mut v, _| {
            let i = dist0.sample(&mut rng);
            let j = (i + 1 + dist1.sample(&mut rng)) % n;

            v.swap(i, j);

            v
        });

        if ab
            .iter()
            .copied()
            .map(|(a, b)| (v[a] + n - v[b]) % n)
            .all(|diff| diff != 1 && diff != n - 1)
        {
            s += 1;
        }

        t += 1;
    }

    eprintln!("{} {}", s, t);

    let ans = s as f64 / t as f64;
    println!("{}", ans);
}
