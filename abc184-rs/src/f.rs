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

fn main() {
    let (n, t) = read_cols!(usize, usize);

    let a = read_vec::<usize>();

    let mut s1 = a.iter().copied().take(n / 2).fold(vec![0], |mut s, aa| {
        let t = s.len();

        for i in 0..t {
            s.push(s[i] + aa);
        }

        s
    });

    let mut s2 = a.iter().copied().skip(n / 2).fold(vec![0], |mut s, aa| {
        let t = s.len();

        for i in 0..t {
            s.push(s[i] + aa);
        }

        s
    });

    s1.sort();
    s2.sort();

    let ans = s1
        .iter()
        .copied()
        .fold((0, s2.len() - 1), |(ans, mut idx), ss| {
            while idx > 0 && s2[idx] + ss > t {
                idx -= 1;
            }

            if s2[idx] + ss > t {
                (ans, idx)
            } else {
                (max(ans, ss + s2[idx]), idx)
            }
        })
        .0;

    println!("{}", ans);
}
