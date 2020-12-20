#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::Itertools;
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
macro_rules! read_tuple {
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
fn read_row<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

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

trait IteratorDpExt: Iterator + Sized {
    fn dp<T, F: FnMut(&Vec<T>, Self::Item) -> T>(self, init: Vec<T>, mut f: F) -> Vec<T> {
        self.fold(init, |mut dp, item| {
            let next = f(&dp, item);
            dp.push(next);
            dp
        })
    }
}

impl<I> IteratorDpExt for I where I: Iterator + Sized {}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Vector {
    x: i64,
    y: i64,
}

impl Vector {
    fn new(p0: (i64, i64), p1: (i64, i64)) -> Self {
        Vector {
            x: p1.0 - p0.0,
            y: p1.1 - p0.1,
        }
    }

    fn dot(self, rhs: Self) -> i64 {
        self.x * rhs.x + self.y * rhs.y
    }

    fn cross(self, rhs: Self) -> i64 {
        self.x * rhs.y - self.y * rhs.x
    }

    fn is_zero(self) -> bool {
        self.x == 0 && self.y == 0
    }

    // for utility
    fn remove_y0(self) -> Self {
        if self.y == 0 {
            Vector {
                x: self.x.signum() * 100000000,
                y: 1,
            }
        } else {
            self
        }
    }
}

fn main() {
    let n: usize = read();

    let xy = read_vec(n, || read_tuple!(i64, i64));

    let (sum_just90, sum_over90) = xy
        .iter()
        .copied()
        .map(|p0| {
            let others = xy
                .iter()
                .copied()
                .map(|p1| Vector::new(p0, p1))
                .filter(|v| !v.is_zero())
                .sorted_by(|v1, v2| {
                    let v1 = v1.remove_y0();
                    let v2 = v2.remove_y0();

                    if v1.y * v2.y < 0 {
                        // negative is bigger
                        v2.y.cmp(&v1.y)
                    } else {
                        0.cmp(&v1.cross(v2))
                    }
                })
                .collect_vec();
            let m = n - 1;
            others
                .iter()
                .copied()
                .scan((1usize, 1usize), |(at90, at180), v| {
                    *at90 = (*at90..)
                        .skip_while(|at90| v == others[at90 % m])
                        .skip_while(|at90| {
                            v.dot(others[at90 % m]) > 0 && v.cross(others[at90 % m]) > 0
                        })
                        .next()
                        .unwrap();
                    *at180 = (*at180..)
                        .skip_while(|at180| v == others[at180 % m])
                        .skip_while(|at180| v.cross(others[at180 % m]) > 0)
                        .next()
                        .unwrap();

                    if v.dot(others[*at90 % m]) == 0 && v.cross(others[*at90 % m]) > 0 {
                        *at90 += 1;
                        Some((1, *at180 - *at90))
                    } else {
                        Some((0, *at180 - *at90))
                    }
                })
                .fold(
                    (0usize, 0usize),
                    |(sum_just90, sum_over90), (just90, over90)| {
                        (sum_just90 + just90, sum_over90 + over90)
                    },
                )
        })
        .fold(
            (0usize, 0usize),
            |(sum_just90, sum_over90), (just90, over90)| (sum_just90 + just90, sum_over90 + over90),
        );
    let sum_under90 = n * (n - 1) * (n - 2) / 6 - sum_just90 - sum_over90;

    println!("{} {} {}", sum_under90, sum_just90, sum_over90);
}
