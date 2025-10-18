#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::*;
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

fn read_time_pair() -> (usize, usize) {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| {
            s.split(':').enumerate().fold(0usize, |u: usize, (i, t)| {
                u + (60 - 59 * i) * t.parse::<usize>().unwrap()
            })
        })
        .collect_tuple()
        .unwrap()
}

fn has_overlap(time1: &(usize, usize), time2: &(usize, usize)) -> bool {
    (time1.1 > time2.0 && time1.0 < time2.1)
        || time1.0 + 24 * 60 < time2.1
        || time2.0 + 24 * 60 < time1.1
}

fn main() {
    let n: usize = read();

    let times = read_vec(n, || read_time_pair());

    let ok_or_not = (0..1 << n)
        .map(|s| {
            let bs = {
                let mut bs = BitSet::new(n);
                bs.buffer_mut()[0] = s;
                bs
            };

            iproduct!((0..n).filter(|&i| bs[i]), (0..n).filter(|&i| bs[i]))
                .filter(|(i, j)| i != j)
                .all(|(i, j)| !has_overlap(&times[i], &times[j]))
        })
        .collect_vec();

    let ans = (1..1 << n).dp(vec![Some(0usize)], |dp, s: usize| {
        std::iter::successors(Some(s), |t| t.checked_sub(1).map(|t| t & s))
            .take_while(|&t| t >= s - t)
            .filter_map(|t| {
                if t == s {
                    if ok_or_not[s] { Some(1) } else { None }
                } else {
                    dp[t].and_then(|d| dp[s - t].map(|d2| d + d2))
                }
            })
            .min()
    })[(1 << n) - 1]
        .unwrap();

    println!("{}", ans);
}
