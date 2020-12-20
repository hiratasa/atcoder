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

trait UsizeExt {
    fn difference(self, rhs: Self) -> Self;
}

impl UsizeExt for usize {
    fn difference(self, rhs: Self) -> Self {
        if self < rhs {
            rhs - self
        } else {
            self - rhs
        }
    }
}

fn main() {
    let (h, w, d) = read_cols!(usize, usize, usize);

    let a = (0..h).map(|_| read_vec::<usize>()).collect::<Vec<_>>();

    let b = a
        .iter()
        .enumerate()
        .fold(vec![(0, 0); h * w + 1], |b, (i, aa)| {
            aa.iter().copied().enumerate().fold(b, |mut b, (j, v)| {
                b[v] = (i, j);

                b
            })
        });

    let c = b
        .iter()
        .copied()
        .enumerate()
        .fold(vec![0; h * w + 1], |mut c, (i, (x, y))| {
            if d < i {
                let (x2, y2) = b[i - d];
                c[i] = c[i - d] + x.difference(x2) + y.difference(y2);
            }

            c
        });

    let q: usize = read();
    for _ in 0..q {
        let (l, r) = read_cols!(usize, usize);

        let ans = c[r] - c[l];

        println!("{}", ans);
    }
}
