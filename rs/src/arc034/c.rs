#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::iter::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;

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
macro_rules! read_tuple {
    ($($t:ty),+) => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

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
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_row<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_col<T: FromStr>(n: usize) -> Vec<T> {
    (0..n).map(|_| read()).collect()
}

#[allow(dead_code)]
fn read_mat<T: FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_row()).collect()
}

#[allow(dead_code)]
fn read_vec<R, F: FnMut() -> R>(n: usize, mut f: F) -> Vec<R> {
    (0..n).map(|_| f()).collect()
}

fn prime_factors(n: usize) -> BTreeMap<usize, usize> {
    (2..)
        .scan(n, |nn, i| {
            if *nn == 1 {
                None
            } else if i * i <= *nn {
                match successors(Some(*nn), |&nn| Some(nn / i))
                    .take_while(|&nn| nn % i == 0)
                    .enumerate()
                    .last()
                {
                    None => Some((i, 0)),
                    Some((d, mm)) => {
                        *nn = mm / i;
                        Some((i, d + 1))
                    }
                }
            } else {
                Some((replace(nn, 1), 1))
            }
        })
        .filter(|&(_p, d)| d > 0)
        .collect()
}

fn main() {
    let (a, b) = read_tuple!(usize, usize);

    if a == b {
        println!("1");
        return;
    }

    let pf = (b + 1..=a)
        .map(|i| prime_factors(i))
        // .inspect(|pf| eprintln!("{:?}", pf))
        .fold(BTreeMap::new(), |mut pf, tpf| {
            tpf.iter().for_each(|(&p, &d)| {
                *pf.entry(p).or_insert(0) += d;
            });
            pf
        });
    const M: usize = 1_000_000_007;
    let ans = pf.values().map(|d| d + 1).fold(1, |prod, d| prod * d % M);
    println!("{}", ans);
}
