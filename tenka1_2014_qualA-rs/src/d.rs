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

fn main() {
    let n: usize = read();

    let b = read_vec(n, || read_tuple!(i64, i64, i64, i64));

    let z = |p: (i64, i64)| {
        if p.1 == 0 {
            if p.0 >= 0 {
                0
            } else {
                2
            }
        } else if p.1 > 0 {
            1
        } else {
            3
        }
    };

    let cross = |p1: (i64, i64), p2: (i64, i64)| p1.0 * p2.1 - p1.1 * p2.0;
    let dot = |p1: (i64, i64), p2: (i64, i64)| p1.0 * p2.0 + p1.1 * p2.1;
    let cmp_angle = |p1: (i64, i64), p2: (i64, i64)| z(p1).cmp(&z(p2)).then(0.cmp(&cross(p1, p2)));

    let b = b
        .into_iter()
        .map(|(x1, y1, x2, y2)| {
            if cross((x1, y1), (x2, y2)) >= 0 {
                ((x1, y1), (x2, y2))
            } else {
                ((x2, y2), (x1, y1))
            }
        })
        .collect_vec();

    let b = b
        .iter()
        .copied()
        .sorted_by(|&(_p1, p2), &(_p3, p4)| cmp_angle(p2, p4))
        .collect_vec();

    let m = b.len();
    assert!(m > 0);
    let ans = (0..m)
        .map(|i| {
            (1..m)
                .fold((1usize, b[i].1), |(s, p), j| {
                    let (p3, p4) = b[(i + j) % m];

                    let cross3 = cross(p3, p);
                    let cross4 = cross(p4, p);
                    if cross3 == 0 && cross4 == 0 {
                        if dot(p, p3) > 0 {
                            (s, p)
                        } else {
                            (s + 1, p4)
                        }
                    } else if cross3 >= 0 && cross4 <= 0 {
                        (s, p)
                    } else {
                        (s + 1, p4)
                    }
                })
                .0
        })
        .min()
        .unwrap();

    println!("{}", ans);
}
